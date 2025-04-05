use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}
