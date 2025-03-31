use scylla::client::session::Session;

use crate::storage;

#[derive(Debug)]
pub struct AppState {
    pub db: Session,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let session = storage::create_session().await?;

        storage::prepare_storage(&session).await?;

        Ok(Self { db: session })
    }
}
