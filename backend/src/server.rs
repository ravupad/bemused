use crate::database::Database;
use crate::request::Request;
use crate::router;
use crate::sled::Sled;
use crate::{error::Error, Future};
use crate::configuration::Configuration;
use futures::{future, Future as _, Stream};
use hyper::server::conn::Http;
use hyper::service::Service;
use hyper::Body;
use hyper::Request as HRequest;
use hyper::Response as HResponse;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use slog::{info, o, Logger};
use std::io;
use std::net::SocketAddr;
use std::ops::Deref;
use std::sync::Arc;
use std::thread;
use tokio_core::net::TcpListener;
use tokio_core::reactor::{Core, Handle};

pub struct ServerInner {
    log: Logger,
    pub database: Database,
    pub sled: Sled,
}

pub struct Server(Arc<ServerInner>);

impl Server {
    pub fn new(log: &Logger, config: &Configuration) -> Result<Self, Error> {
        let log = log.new(o!());
        info!(log, "Connecting to postgres");
        let pg_man = PostgresConnectionManager::new(
            config.postgres.to_owned(),
            TlsMode::None,
        )
        .unwrap();
        let pg_pool = r2d2::Pool::builder().max_size(16).build(pg_man).unwrap();
        let sled = Sled::new("database");
        Ok(Server(Arc::new(ServerInner {
            log,
            database: Database::new(pg_pool),
            sled,
        })))
    }

    pub fn start(self, port: u16, threads: usize) {
        for _ in 1..threads {
            let server = self.clone();
            thread::spawn(move || server_thread(server, port));
        }
        server_thread(self, port);
    }

    pub fn clone(&self) -> Self {
        Server(self.0.clone())
    }

    pub fn handle(self, rc: Request) -> Future<hyper::Response<hyper::Body>> {
        let method = rc.method.clone();
        let uri = rc.uri.clone();
        let log = self.log.new(o!());
        Box::new(
            router::router(self.log.new(o!()), self, rc)
                .map(|res| res.into_response())
                .or_else(|error| {
                    future::ok(
                        hyper::Response::builder()
                            .status(error.error_code.status_code())
                            .body(hyper::Body::from(serde_json::to_string(&error).unwrap()))
                            .unwrap(),
                    )
                })
                .inspect(move |res| {
                    info!(log, "{} {} {}", method, uri, res.status());
                }),
        )
    }
}

fn server_thread(server: Server, port: u16) {
    let mut http = Http::new();
    http.pipeline_flush(true);
    let mut core = Core::new().expect("tokio core could not be created");
    let handle = core.handle();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let tcp = reuse_listener(&addr, &handle).expect("couldn't bind to addr");
    let server = tcp
        .incoming()
        .for_each(move |(sock, _addr)| {
            let _ = sock.set_nodelay(true);
            let server = server.clone();
            let conn = http
                .serve_connection(sock, server)
                .map_err(|e| eprintln!("connection error: {}", e));
            handle.spawn(conn);
            Ok(())
        })
        .map_err(|e| eprintln!("accept error: {}", e));
    core.run(server).expect("server");
}

fn reuse_listener(addr: &SocketAddr, handle: &Handle) -> io::Result<TcpListener> {
    let builder = match *addr {
        SocketAddr::V4(_) => net2::TcpBuilder::new_v4()?,
        SocketAddr::V6(_) => net2::TcpBuilder::new_v6()?,
    };
    #[cfg(unix)]
    {
        use net2::unix::UnixTcpBuilderExt;
        if let Err(e) = builder.reuse_port(true) {
            eprintln!("error setting SO_REUSEPORT: {}", e);
        }
    }
    builder.reuse_address(true)?;
    builder.bind(addr)?;
    builder
        .listen(1024)
        .and_then(|l| TcpListener::from_listener(l, addr, handle))
}

impl Deref for Server {
    type Target = ServerInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Service for Server {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Error;
    type Future = Future<HResponse<Self::ResBody>>;

    fn call(&mut self, req: HRequest<Self::ReqBody>) -> Self::Future {
        let server = self.clone();
        let (parts, body) = req.into_parts();
        let res = body
            .fold(Vec::new(), |mut acc, chunk| {
                acc.extend_from_slice(&*chunk);
                future::ok::<_, hyper::error::Error>(acc)
            })
            .map_err(|e| Error::internal(&e))
            .and_then(move |body: Vec<u8>| server.handle(Request::new(parts, body)));
        Box::new(res)
    }
}
