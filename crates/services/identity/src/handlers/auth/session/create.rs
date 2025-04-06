use std::{env, str::FromStr, sync::Arc};

use auth::session::store::{SessionManager, impls::scylla::ScyllaSessionStore};
use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use crate::model::user::LoginPayload;

pub async fn handler(
    Extension(session_manager): Extension<Arc<SessionManager<ScyllaSessionStore>>>,
    cookies: CookieJar,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    let user_id = reqwest::Client::new()
        .get(format!(
            "{}/internal/user/{}",
            env::var("USERS_SERVICE_URI").unwrap(),
            payload.username
        ))
        .body(payload.password)
        .send()
        .await
        .unwrap();
    if user_id.status() != StatusCode::OK {
        return (StatusCode::BAD_REQUEST, "wrong password.").into_response();
    }

    let user_id = Uuid::from_str(&user_id.text().await.unwrap()).unwrap();

    let session_cookie = session_manager
        .create_session(&Uuid::new_v4(), &user_id)
        .await
        .unwrap();

    (StatusCode::OK, cookies.add(session_cookie)).into_response()
}
