use std::sync::Arc;

use axum::{Router, routing::post};

use crate::state::AppState;

pub mod register;
pub mod session;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/register", post(register::handler))
        .route("/auth/session/create", post(session::create))
    // .route("/auth/session/refresh", post(refresh::handler))
}
