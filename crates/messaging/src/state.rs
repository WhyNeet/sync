use scylla::client::session::Session;
use tokio::sync::broadcast::{self, Sender};

use crate::storage;

#[derive(Debug)]
pub struct AppState {
    pub db: Session,
    pub channel_tx: Sender<String>,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let session = storage::create_session().await?;

        storage::prepare_storage(&session).await?;

        let (channel_tx, _) = broadcast::channel(1);

        Ok(Self {
            db: session,
            channel_tx,
        })
    }
}
