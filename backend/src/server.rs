use crate::configuration::Configuration;
use crate::database::Database;
use crate::error::Error;
use crate::response::response_from_result;
use crate::router;
use crate::sled::Sled;
use futures::StreamExt;
use http::Request as Request;
use http::Response;
use http::request::Parts;
use hyper::Body;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use slog::{info, o, warn, Logger};
use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::str::Split;
use std::iter::Map;

pub struct ServerInner {
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

async fn handle(server: Server, request: Request<Body>) -> Response<Body> {
    let logger = server.log.new(o!());
    let response = router::router(logger, server, request).await;
    response_from_result(response)
}



impl tower_service::Service<Request<Body>> for Server {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let server: Server = self.clone();
        let future = async move {
            Ok(handle(server, req).await)
        };
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

pub fn get_query_param(parts: &Parts, key: &str) -> Option<String> {
    parts.uri.query()?
        .split('&')
        .map(|kv: &str| kv.split('=').collect())
        .filter(|kv: &Vec<&str>| kv.len() == 2)
        .filter(|kv: &Vec<&str>| kv[0] == key)
        .map(|kv: Vec<&str>| kv[1].to_string())
        .nth(0)
}
