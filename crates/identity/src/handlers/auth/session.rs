use std::sync::Arc;

use auth::{
    hashing::{Hasher, initialize_hasher},
    session::store::{SessionManager, impls::scylla::ScyllaSessionStore},
};
use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;
use serde_json::json;
use uuid::Uuid;

use crate::{model::user::LoginPayload, state::AppState};

pub async fn create(
    State(state): State<Arc<AppState>>,
    Extension(session_manager): Extension<Arc<SessionManager<ScyllaSessionStore>>>,
    cookies: CookieJar,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    let user = state
        .db
        .query_unpaged(
            r#"SELECT id FROM ks.users_by_username WHERE username = ? LIMIT 1"#,
            (payload.username,),
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

    let user_password = state
        .db
        .query_unpaged(
            r#"SELECT password FROM ks.users WHERE id = ? LIMIT 1"#,
            (user_id,),
        )
        .await
        .unwrap()
        .into_rows_result()
        .unwrap()
        .first_row::<(String,)>()
        .unwrap()
        .0;

    if !initialize_hasher()
        .check_str(payload.password.as_bytes(), &user_password)
        .unwrap()
    {
        return (
            StatusCode::BAD_REQUEST,
            json!({ "kind": "error", "data": "Wrong password." }).to_string(),
        )
            .into_response();
    }

    let session_cookie = session_manager
        .create_session(&Uuid::new_v4(), &user_id)
        .await
        .unwrap();

    (StatusCode::OK, cookies.add(session_cookie)).into_response()
}
