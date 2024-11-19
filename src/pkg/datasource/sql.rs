use std::time::Duration;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use tokio::time::sleep;
use tracing::log;
use crate::CONFIG;

pub async fn load_sql() -> Pool<Postgres> {
    const MAX_ATTEMPTS: i32 = 30;
    const RETRY_DELAY: Duration = Duration::from_secs(5);
    
    let mut attempts = 0;
    
    while attempts < MAX_ATTEMPTS {
        attempts += 1;
        
        match PgPoolOptions::new()
            .max_connections(50)
            .min_connections(10)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(60))
            .max_lifetime(Duration::from_secs(3600))
            .connect(CONFIG.db_uri.as_str())
            .await {
            Ok(pool) => {
                match sqlx::query("SELECT 1").execute(&pool).await {
                    Ok(_) => {
                        log::info!("Successfully connected to the database");
                        return pool;
                    }
                    Err(err) => {
                        log::warn!(
                            "Failed to ping database (attempt {}/{}) : {}",
                            attempts,
                            MAX_ATTEMPTS,
                            err
                        );
                    }
                }
            }
            Err(err) => {
                log::warn!(
                    "Failed to connect to database (attempt {}/{}) : {}",
                    attempts,
                    MAX_ATTEMPTS,
                    err
                );
            }
        }

        sleep(RETRY_DELAY).await;
    }

    panic!("Failed to connect to database after {} attempts", MAX_ATTEMPTS);
}