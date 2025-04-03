use std::{str::FromStr, sync::Arc};

use axum::{
    Extension,
    extract::{FromRequestParts, Request},
    http::{StatusCode, header, request::Parts},
    middleware::Next,
    response::Response,
};
use cookie::Cookie;

use crate::session::{
    data::SessionData,
    store::{SessionManager, SessionStore},
};

pub type SessionId = String;

// --- Extractor to get session data in handlers ---
#[derive(Debug, Clone)]
pub struct Session(pub SessionData); // Wrap the actual data

impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let extensions = &parts.extensions;
        let maybe_data = extensions.get::<Option<SessionData>>().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Session layer missing or failed",
        ))?;

        match maybe_data {
            Some(data) => Ok(Session(data.clone())),
            None => Err((StatusCode::UNAUTHORIZED, "Not authenticated")),
        }
    }
}

pub async fn session_middleware<Store: SessionStore>(
    Extension(session_manager): Extension<Arc<SessionManager<Store>>>, // Get manager via extension
    mut request: Request,
    next: Next,
) -> Response {
    let mut session_id: Option<SessionId> = None;

    // 1. Try to load session from request cookie
    let cookies = request
        .headers()
        .get_all(header::COOKIE)
        .into_iter()
        .filter_map(|v| v.to_str().ok())
        .filter_map(|cookie| Cookie::from_str(cookie).ok())
        .collect::<Vec<_>>();

    if let Some(cookie_header) = cookies.iter().find(|c| c.name() == "sid") {
        let sid = cookie_header.value();
        let Ok(sid) = session_manager.load_session(sid).await else {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("invalid session id.".into())
                .unwrap();
        };
        session_id = Some(sid);
    } else {
    }

    request.extensions_mut().insert(session_id);

    let response = next.run(request).await;

    response
}
