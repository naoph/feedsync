use actix_web::{get, post, web, Responder};

use super::req::*;
use super::resp::*;
use super::{State, UrlState};

#[post("/channel")]
pub async fn create_channel(data: web::Data<State>, request: web::Json<CreateChannelReq>) -> impl Responder {
    debug!("POST /channel ({})", request.remote.as_str());
    let url_map = data.url_map.clone();
    let mut guard = url_map.lock().await;
    match guard.get(&request.remote) {
        None => {
            guard.insert(request.remote.clone(), UrlState::InProgress);
            tokio::spawn(async move {
                let url_map = data.url_map.clone();
                info!("Adding {} ...", &request.remote);
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                let result_state = UrlState::Success { slug: "placeholder".to_string() };
                url_map.lock().await.insert(request.remote.clone(), result_state);
            });
            web::Json(CreateChannelResp::Initiated)
        },
        Some(UrlState::InProgress) => {
            web::Json(CreateChannelResp::InProgress)
        },
        Some(UrlState::Success { slug }) => {
            web::Json(CreateChannelResp::Success { slug: slug.clone() })
        },
        Some(UrlState::Failure { error }) => {
            web::Json(CreateChannelResp::Failure { error: error.clone() })
        },
    }
}
