use auth::AuthEvent;
use serde::{Deserialize, Serialize};

pub mod auth;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    kind: EventKind,
}

impl Event {
    pub fn new(kind: EventKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventKind {
    AuthEvent(AuthEvent),
}
