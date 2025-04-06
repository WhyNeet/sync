use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

pub mod credentials;
pub mod me;
pub mod new;
pub mod user;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/me", get(me::handler))
        .route("/register", post(new::handler))
        .route("/user/{username}", get(user::handler))
        .route("/internal/user/{username}", get(credentials::handler))
}
