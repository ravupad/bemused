mod configuration;
mod database;
mod error;
mod model;
mod response;
mod router;
mod serde;
mod server;
mod service;
mod sled;

use self::configuration::Configuration;
use self::error::Error;
use self::server::Server;
use crate::server::start;
use crate::server::get_logger;
use dotenv::dotenv;
use slog::{info};

type Result<T> = std::result::Result<T, Error>;

fn main() {
    dotenv().ok();
    let configuration = Configuration::new();
    let log = get_logger(&configuration);
    info!(log, "Starting http server");
    let server = Server::new(log, configuration);
    let future = start(server);
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(future);
}

