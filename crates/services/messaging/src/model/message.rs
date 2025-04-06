use scylla::{DeserializeRow, SerializeRow, value::CqlTimeuuid};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, DeserializeRow, SerializeRow)]
pub struct ChatMessage {
    pub id: CqlTimeuuid,
    pub chat_id: Uuid,
    pub content: String,
    pub user_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePayload {
    pub id: String,
    pub content: String,
    pub user_id: Uuid,
    pub chat_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatMessageInput {
    pub content: String,
}
