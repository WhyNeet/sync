mod health;

use axum::{Router, routing::get};
pub use health::healthcheck;

pub fn default_router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::<S>::new().route("/health", get(health::healthcheck))
}
