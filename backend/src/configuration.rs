use std::env;

pub struct Configuration {
    pub port: u16,
    pub postgres: String,
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            port: env::var("SERVER_PORT").unwrap().parse().unwrap(),
            postgres: env::var("POSTGRES_CONNECTION_STRING").unwrap(),
        }
    }
}
