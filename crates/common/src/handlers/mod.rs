mod health;
use axum::{Router, routing::get};
pub use health::healthcheck;

pub fn default_router() -> Router {
    Router::new().route("/health", get(health::healthcheck))
}
