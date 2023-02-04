use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct CreateChannelReq {
    pub remote: Url,
}
