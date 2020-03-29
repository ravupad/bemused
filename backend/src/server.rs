use crate::configuration::Configuration;
use crate::database::Database;
use crate::error::Error;
use crate::request::Request;
use crate::router;
use crate::sled::Sled;
use futures::StreamExt;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use slog::{info, o, warn, Logger};
use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use crate::response::response_from_result;

pub struct ServerInner {
    log: Logger,
    pub pool: r2d2::Pool<PostgresConnectionManager>,
    pub database: Database,
    pub sled: Sled,
}

pub struct Server(Arc<ServerInner>);

impl Server {
    pub fn new(logger: &Logger, config: &Configuration) -> Result<Self, Error> {
        let log = logger.new(o!());
        info!(log, "Connecting to postgres");
        let pg_man =
            PostgresConnectionManager::new(config.postgres.to_owned(), TlsMode::None).unwrap();
        let pg_pool = r2d2::Pool::builder().max_size(16).build(pg_man).unwrap();
        let sled = Sled::new("database");
        Ok(Server(Arc::new(ServerInner {
            log,
            pool: pg_pool.clone(),
            database: Database::new(pg_pool.clone()),
            sled,
        })))
    }

    pub async fn start(self, port: u16) {
        let http = hyper::server::conn::Http::new();
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let mut listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        let mut incoming = listener.incoming();
        info!(self.log, "started listener");
        while let Some(stream) = incoming.next().await {
            match stream {
                Ok(stream) => match http.serve_connection(stream, self.clone()).await {
                    Err(e) => warn!(self.log, "http service error: {:?}", e),
                    _ => (),
                },
                Err(e) => warn!(self.log, "socket connection accept failed: {:?}", e),
            }
        }
    }

    pub fn clone(&self) -> Self {
        Server(self.0.clone())
    }
}

async fn handle(server: Server, req: Request) -> http::Response<hyper::Body> {
    response_from_result(router::router(server.log.new(o!()), server, req).await)
}

impl Deref for Server {
    type Target = ServerInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl tower_service::Service<http::Request<hyper::Body>> for Server {
    type Response = http::Response<hyper::Body>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: http::Request<hyper::Body>) -> Self::Future {
        let server: Server = self.clone();
        let (parts, mut body) = req.into_parts();
        let future = async move {
            let mut raw_body = Vec::new();
            loop {
                if let Some(chunk) = body.next().await {
                    match chunk {
                        Ok(bytes) => raw_body.extend_from_slice(&bytes),
                        Err(e) => {
                            warn!(server.log, "socket read error: {}", e);
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
            let request = Request::new(parts, raw_body);
            Ok(handle(server, request).await)
        };
        Box::pin(future)
    }
}
