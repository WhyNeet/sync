use scylla::{DeserializeRow, SerializeRow};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, SerializeRow, DeserializeRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub display_name: String,
    pub avatar_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPayload {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub avatar_id: Uuid,
}

impl UserPayload {
    pub fn from_user(user: &User) -> Self {
        Self {
            id: user.id.clone(),
            avatar_id: user.avatar_id.clone(),
            display_name: user.display_name.clone(),
            username: user.username.clone(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
}
