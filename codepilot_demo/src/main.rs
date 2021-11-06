use actix_web::{
    get,
    HttpServer,
};

use std::io::Result;
pub mod orm;

#[get("hello")]
async fn hello() -> &'static str {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> Result<()> {
    let server = HttpServer::new(|| actix_web::App::new().service(hello));
    server.bind("128.0.0.1:8080")?.run().await
}
