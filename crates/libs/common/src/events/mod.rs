use auth::AuthEvent;
use serde::{Deserialize, Serialize};

pub mod auth;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    kind: EventKind,
}

impl Event {
    pub fn new(kind: EventKind) -> Self {
        Self { kind }
    }

    pub fn kind(&self) -> &EventKind {
        &self.kind
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventKind {
    AuthEvent(AuthEvent),
}
