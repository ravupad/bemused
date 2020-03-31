use std::env;

pub struct Configuration {
    pub port: u16,
    pub postgres: String,
    pub log_file: String,
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            port: env::var("SERVER_PORT").unwrap().parse().unwrap(),
            postgres: env::var("POSTGRES_CONNECTION_STRING").unwrap(),
            log_file: env::var("LOG_FILE").unwrap(),
        }
    }
}
