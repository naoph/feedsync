use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum CreateChannelResp {
    Initiated,
    InProgress,
    Success { slug: String },
    Failure { error: String },
}
