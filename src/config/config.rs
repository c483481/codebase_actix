use std::env;

pub struct Config {
    pub port: u16,
    pub db_uri: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        Self {
            port: env::var("APP_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            db_uri: Self::load_db_url()
        }
    }

    fn load_db_url() -> String {
        let db_host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
        let db_port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
        let db_user = env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
        let db_pass = env::var("DB_PASS").unwrap_or_else(|_| "postgres".to_string());
        let db_name = env::var("DB_NAME").unwrap_or_else(|_| "postgres".to_string());

        format!("postgres://{db_user}:{db_pass}@{db_host}:{db_port}/{db_name}")
    }
}
