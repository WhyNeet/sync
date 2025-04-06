use axum::response::IntoResponse;
use serde_json::json;

pub async fn healthcheck() -> impl IntoResponse {
    json!({ "status": "running" }).to_string()
}
