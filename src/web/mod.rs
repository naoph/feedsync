mod api;

use actix_web::{HttpServer, App};

pub async fn run(host: impl ToString, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api::ping)
    })
    .bind((host.to_string(), port))?
    .run()
    .await
}
