use actix_web::{middleware::Logger, App, HttpServer};
use app_info::manifest_app;
use config::config::Config;
use env_logger::Env;
use std::time::Instant;

mod app_info;
mod config;
mod response;
mod pkg;

use lazy_static::lazy_static;

lazy_static! {
    static ref START_TIME: Instant = Instant::now();
    static ref CONFIG: Config = Config::new();
}

const APP_NAME: &str = "codebase actix-web";
const APP_VERSION: &str = "1.0.0";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    println!("🚀 Server running at http://localhost:{}", CONFIG.port);
    println!("{}", CONFIG.db_uri);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%a '%r' %s %b '%{Referer}i' %Dms"))
            .service(manifest_app)
    })
    .bind(("127.0.0.1", CONFIG.port))?
    .run()
    .await
}
