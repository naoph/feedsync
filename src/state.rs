use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use url::Url;

use super::PgPool;

#[derive(Clone, Debug)]
pub enum UrlState {
    InProgress,
    Success { slug: String },
    Failure { error: String },
}

#[derive(Clone)]
pub struct State {
    pub pool: PgPool,
    pub url_map: Arc<Mutex<HashMap<Url, UrlState>>>,
}

impl State {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            url_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
