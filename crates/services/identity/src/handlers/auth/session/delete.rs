use std::sync::Arc;

use auth::session::store::Session;
use axum::{extract::State, response::IntoResponse};
use axum_extra::extract::CookieJar;
use cookie::Cookie;
use reqwest::StatusCode;

use crate::state::AppState;

pub async fn handler(
    session: Session,
    cookies: CookieJar,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    state
        .db
        .query_unpaged(r#"DELETE FROM ks.sessions WHERE id = ?"#, (session.0.id,))
        .await
        .unwrap();

    (StatusCode::OK, cookies.remove(Cookie::from("sid")))
}
