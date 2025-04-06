use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;

use crate::state::AppState;

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    let user = state
        .db
        .query_unpaged(
            r#"SELECT id FROM ks.users_by_username WHERE username = ? LIMIT 1"#,
            (username,),
        )
        .await
        .unwrap();
    let user = user.into_rows_result().unwrap().first_row::<(Uuid,)>();
    let user_id = if let Ok(user) = user {
        user.0
    } else {
        return (
            StatusCode::BAD_REQUEST,
            json!({ "kind": "error", "data": "User does not exist." }).to_string(),
        )
            .into_response();
    };

    (StatusCode::OK, user_id.to_string()).into_response()
}
