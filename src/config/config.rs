use std::env;

pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        Self {
            port: env::var("APP_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
        }
    }
}
