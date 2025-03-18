use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub content: String,
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatMessageInput {
    pub content: String,
}
