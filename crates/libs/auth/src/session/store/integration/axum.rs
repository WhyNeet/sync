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
    store::{Session, SessionManager, SessionStore},
};

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
    let mut session: Option<SessionData> = None;

    let cookies = request
        .headers()
        .get_all(header::COOKIE)
        .into_iter()
        .filter_map(|v| v.to_str().ok())
        .filter_map(|cookie| Cookie::from_str(cookie).ok())
        .collect::<Vec<_>>();

    if let Some(cookie_header) = cookies.iter().find(|c| c.name() == "sid") {
        let sid = cookie_header.value();
        session = session_manager.load_session(sid).await.ok();
    } else {
    }

    request.extensions_mut().insert(session);

    let response = next.run(request).await;

    response
}
