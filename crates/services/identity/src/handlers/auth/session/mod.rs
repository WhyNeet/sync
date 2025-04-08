use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, post},
};

use crate::state::AppState;

pub mod create;
pub mod delete;
pub mod validate;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/session/create", post(create::handler))
        .route("/internal/validate", post(validate::handler))
        .route("/session", delete(delete::handler))
}
