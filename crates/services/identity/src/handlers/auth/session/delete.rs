use std::sync::Arc;

use auth::session::store::Session;
use axum::{extract::State, response::IntoResponse};
use axum_extra::extract::CookieJar;
use common::events::{Event, EventKind, auth::AuthEvent};
use cookie::Cookie;
use reqwest::StatusCode;

use crate::state::AppState;

pub async fn handler(
    session: Session,
    cookies: CookieJar,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    state
        .bus_producer
        .send(Event::new(EventKind::AuthEvent(
            AuthEvent::SessionTerminated(session.0.id),
        )))
        .unwrap();

    state
        .db
        .query_unpaged(r#"DELETE FROM ks.sessions WHERE id = ?"#, (session.0.id,))
        .await
        .unwrap();

    (StatusCode::OK, cookies.remove(Cookie::from("sid")))
}
