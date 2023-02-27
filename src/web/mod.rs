mod api;
mod req;
mod resp;

use std::{sync::Arc, collections::HashMap};

use actix_web::{HttpServer, App, web};
use tokio::sync::Mutex;
use url::Url;

#[derive(Clone, Debug)]
pub enum UrlState {
    InProgress,
    Success { slug: String },
    Failure { error: String },
}

#[derive(Clone)]
pub struct State {
    url_map: Arc<Mutex<HashMap<Url, UrlState>>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            url_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

pub async fn run(host: impl ToString, port: u16) -> std::io::Result<()> {
    let state = State::new();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(api::create_channel)
    })
    .bind((host.to_string(), port))?
    .run()
    .await
}
