use std::sync::Arc;

use auth::session::store::{SessionManager, impls::scylla::ScyllaSessionStore};
use axum::{Extension, http::StatusCode, response::IntoResponse};

pub async fn handler(
    Extension(session_manager): Extension<Arc<SessionManager<ScyllaSessionStore>>>,
    body: String,
) -> impl IntoResponse {
    let session = session_manager.load_session_unverified(&body).await;
    let Ok(session) = session else {
        println!("validation failed: {session:?}");
        return (StatusCode::BAD_REQUEST, "invalid session id.").into_response();
    };

    (StatusCode::OK, session.user_id.to_string()).into_response()
}
