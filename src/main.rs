#![allow(dead_code)]

#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::scope;
use actix_web::{App, HttpServer};
mod api;
mod db;
mod error;
mod logger;
mod models;
mod schema;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    logger::init_fern_logger().unwrap_or_default();
    let addr = db::init_dba();
    let bind_host = dotenv::var("BIND_ADDRESS").unwrap_or("127.0.0.1:8000".to_string());
    println!("Starting http server:  http://{}", bind_host);
    HttpServer::new(move || {
        App::new()
            .data(addr.clone())
            .wrap(Logger::default())
            .wrap(Cors::default())
            .service(
                scope("/api")
                    .service(api::user::signin)
                    .service(api::user::signup),
            )
    })
    .bind(&bind_host)?
    .run()
    .await
}
