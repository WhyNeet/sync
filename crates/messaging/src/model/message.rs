use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub id: Uuid,
    pub content: String,
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct ChatMessageInput {
    pub content: String,
}
