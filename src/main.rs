#[macro_use] extern crate diesel;

mod models;
mod schema;
mod web;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    web::run("127.0.0.1", 8080).await.unwrap();
}
