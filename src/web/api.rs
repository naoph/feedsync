use actix_web::{get, post, web, Responder};

use super::req::*;
use super::resp::*;

#[post("/channel")]
pub async fn create_channel(request: web::Json<CreateChannelReq>) -> impl Responder {
    info!("Creating channel with remote {}", request.remote.as_str());
    web::Json(CreateChannelResp::Initiated)
}
