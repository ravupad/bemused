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
use dotenv::dotenv;
use slog::{info, o};
use std::fs::OpenOptions;
use slog::Drain;
use slog::Logger;
use slog_term::PlainDecorator;
use slog_async::Async;

type Result<T> = std::result::Result<T, Error>;

fn main() {
    dotenv().ok();
    let configuration = Configuration::new();
    let log = get_logger(&configuration);
    info!(log, "Logging ready!");
    let server = Server::new(&log, &configuration).expect("server");
    info!(log, "Starting http server");
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(server.start(configuration.port));
}

fn get_logger(configuration: &Configuration) -> Logger {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&configuration.log_file)
        .unwrap();
    let file_decorator = PlainDecorator::new(file);
    let file_drain = slog_term::CompactFormat::new(file_decorator).build();
    let term_decorator = slog_term::TermDecorator::new().build();
    let term_drain = slog_term::CompactFormat::new(term_decorator).build();
    let combined_drain = slog::Duplicate(file_drain, term_drain).fuse();
    let final_drain = Async::new(combined_drain).build().fuse();
    Logger::root(final_drain, o!())
}
