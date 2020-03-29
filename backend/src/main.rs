mod configuration;
mod database;
mod error;
mod model;
mod request;
mod response;
mod router;
mod serde;
mod server;
mod service;
mod sled;

use self::configuration::Configuration;
use self::error::Error;
use self::server::Server;
use dotenv::dotenv;
use slog::{info, o, Drain};

type Result<T> = std::result::Result<T, Error>;

fn main() {
    dotenv().ok();
    let configuration = Configuration::new();
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, o!());
    info!(log, "Logging ready!");
    let server = Server::new(&log, &configuration).expect("server");
    info!(log, "Starting http server");
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(server.start(configuration.port));
}
