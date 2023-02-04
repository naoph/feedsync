mod api;
mod req;
mod resp;

use actix_web::{HttpServer, App};

pub async fn run(host: impl ToString, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api::create_channel)
    })
    .bind((host.to_string(), port))?
    .run()
    .await
}
