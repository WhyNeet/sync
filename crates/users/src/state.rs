use std::sync::Arc;

use crate::storage;
use scylla::client::session::Session;

#[derive(Debug)]
pub struct AppState {
    pub db: Arc<Session>,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let session = storage::create_session().await?;

        storage::prepare_storage(&session).await?;

        Ok(Self {
            db: Arc::new(session),
        })
    }
}
