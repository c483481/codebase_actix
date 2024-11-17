use actix_web::{middleware::Logger, App, HttpServer};
use app_info::manifest_app;
use env_logger::Env;
mod response;
mod app_info;
use std::time::Instant;

use lazy_static::lazy_static;

lazy_static! {
    static ref START_TIME: Instant = Instant::now();
}

const APP_NAME: &str = "codebase actix-web";
const APP_VERSION: &str = "1.0.0";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    println!("🚀 Server running at http://localhost:3000");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%a '%r' %s %b '%{Referer}i' %Dms"))
            .service(manifest_app)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}