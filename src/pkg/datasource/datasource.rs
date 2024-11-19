use sqlx::{Pool, Postgres};
use crate::pkg::datasource::sql::load_sql;

pub struct Datasource {
    pub sql: Pool<Postgres>
}

impl Datasource {
    pub async fn new() -> Self {
        Datasource {
            sql: load_sql().await
        }
    }
}