mod api;
mod req;
mod resp;

use actix_web::{HttpServer, App, web};

use crate::state::State;

pub async fn run(host: impl ToString, port: u16, state: State) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(api::create_channel)
    })
    .bind((host.to_string(), port))?
    .run()
    .await
}
