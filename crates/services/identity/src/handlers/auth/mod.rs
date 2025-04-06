use std::sync::Arc;

use axum::{Router, routing::post};

use crate::state::AppState;

pub mod session;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().merge(session::router())
}
