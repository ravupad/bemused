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
use futures::future::Future as SFuture;
use slog::{info, o, Drain};
use dotenv::dotenv;

type Result<T> = std::result::Result<T, Error>;
type Future<T> = Box<SFuture<Item = T, Error = Error> + Send>;

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
    server.start(configuration.port, num_cpus::get());
}
