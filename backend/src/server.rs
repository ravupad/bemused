use crate::configuration::Configuration;
use crate::database::Database;
use crate::response::response_from_error;
use crate::router;
use crate::sled::Sled;
use futures::Stream;
use futures::StreamExt;
use futures::FutureExt;
use http::Request;
use http::Response;
use hyper::server::conn::Http;
use hyper::Body;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use slog::{info, o, warn};
use slog::Drain;
use slog::Logger;
use slog_async::Async;
use slog_term::FullFormat;
use slog_term::PlainDecorator;
use std::fs::OpenOptions;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::signal;
use tower_service::Service;
use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use uuid::Uuid;

pub struct ServerInner {
    port: u16,
    log: Logger,
    pub pool: r2d2::Pool<PostgresConnectionManager>,
    pub database: Database,
    pub sled: Sled,
}

pub struct Server(Arc<ServerInner>);

impl Deref for Server {
    type Target = ServerInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Clone for Server {
    fn clone(&self) -> Self {
        Server(self.0.clone())
    }
}

impl Server {
    pub fn new(logger: Logger, config: Configuration) -> Self {
        info!(logger, "Connecting to postgres");
        let pg_man = PostgresConnectionManager::new(
            config.postgres.to_owned(), TlsMode::None).unwrap();
        let pg_pool = r2d2::Pool::builder()
            .max_size(16)
            .build(pg_man)
            .unwrap();
        let sled = Sled::new("database");
        Server(Arc::new(ServerInner {
            port: config.port,
            log: logger,
            pool: pg_pool.clone(),
            database: Database::new(pg_pool.clone()),
            sled,
        }))
    }
}

pub async fn start(server: Server) {
    let http = Http::new();
    let addr = SocketAddr::from(([0, 0, 0, 0], server.port));
    let listener = TcpListener::bind(&addr).await.unwrap();
    info!(server.log, "starting listener");
    let cancellable_stream = CancellableStream {
        stream: listener,
        cancel_future: signal::ctrl_c().map(|_| ()),
    };
    cancellable_stream.for_each_concurrent(10, |stream| {
        handle_new_connection(server.clone(), &http, stream)
    }).await;
    info!(server.log, "shutting down server");
}

async fn handle_new_connection(server: Server, http: &Http, stream: std::io::Result<TcpStream>) {
    let log = server.log.new(o!());
    let stream = match stream {
        Ok(stream) => stream,
        Err(err) => {
            warn!(log, "Tcp Error: {:?}", err);
            return ();
        }
    };
    let result = http.serve_connection(stream, server).await;
    match result {
        Err(err) => warn!(log, "Http Error: {:?}", err),
        _ => (),
    }
}

async fn handle_request(server: Server, request: Request<Body>) -> Response<Body> {
    let logger = server
        .log
        .new(o!("RequestId" => Uuid::new_v4().to_string()));
    let response = router::router(logger.clone(), server, request).await;
    match response {
        Ok(body) => {
            info!(logger, "Request Successful");
            body
        }
        Err(error) => {
            warn!(logger, "Server Error: {:?}", &error);
            response_from_error(error)
        }
    }
}

impl Service<Request<Body>> for Server {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let server: Server = self.clone();
        let future = async move { Ok(handle_request(server, req).await) };
        Box::pin(future)
    }
}

pub async fn get_body(mut body: Body) -> Vec<u8> {
    let mut raw_body = Vec::new();
    loop {
        if let Some(chunk) = body.next().await {
            match chunk {
                Ok(bytes) => raw_body.extend_from_slice(&bytes),
                _ => break,
            }
        } else {
            break;
        }
    }
    raw_body
}

#[allow(dead_code)]
pub fn get_query_param<'a>(query: &'a str, key: &str) -> Option<&'a str> {
    query
        .split('&')
        .map(|kv: &str| kv.split('=').collect())
        .filter(|kv: &Vec<&str>| kv.len() == 2)
        .filter(|kv: &Vec<&str>| kv[0] == key)
        .map(|kv: Vec<&str>| kv[1])
        .nth(0)
}

pub fn get_path(path: &str, offset: usize, len: usize) -> Vec<&str> {
    path.split('/').skip(1).skip(offset).take(len).collect()
}


pub fn get_logger(configuration: &Configuration) -> Logger {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .append(true)
        .open(&configuration.log_file)
        .unwrap();
    let decorator = PlainDecorator::new(file);
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let async_drain = match configuration.terminal_log {
        false => Async::new(drain).build().fuse(),
        true => {
            let term_decorator = slog_term::TermDecorator::new().build();
            let term_drain = FullFormat::new(term_decorator).build();
            let drain = slog::Duplicate(drain, term_drain).fuse();
            Async::new(drain).build().fuse()
        }
    };
    Logger::root(async_drain, o!())
}

struct CancellableStream<St, Fut> {
    stream: St,
    cancel_future: Fut,
}

impl<St, Fut> Stream for CancellableStream<St, Fut>
    where
        St: Stream,
        Fut: Future<Output=()>
{
    type Item = St::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<St::Item>> {
        match unsafe {
            self.as_mut().map_unchecked_mut(|s| &mut s.cancel_future)
        }.poll(cx) {
            Poll::Ready(_) => Poll::Ready(None),
            Poll::Pending => unsafe {
                self.as_mut().map_unchecked_mut(|s| &mut s.stream)
            }.poll_next(cx)
        }
    }
}