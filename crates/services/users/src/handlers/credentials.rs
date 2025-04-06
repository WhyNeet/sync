use std::sync::Arc;

use auth::hashing::{Hasher, initialize_hasher};
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
    input_password: String,
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

    let user = state
        .db
        .query_unpaged(
            r#"SELECT password FROM ks.users WHERE id = ? LIMIT 1"#,
            (user_id,),
        )
        .await
        .unwrap();
    let user_password = user
        .into_rows_result()
        .unwrap()
        .first_row::<(String,)>()
        .unwrap()
        .0;

    if !initialize_hasher()
        .check_str(input_password.as_bytes(), &user_password)
        .unwrap()
    {
        return (
            StatusCode::BAD_REQUEST,
            json!({ "kind": "error", "data": "Wrong password." }).to_string(),
        )
            .into_response();
    }

    (StatusCode::OK, user_id.to_string()).into_response()
}
