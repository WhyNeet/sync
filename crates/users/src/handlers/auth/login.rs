use std::{env, sync::Arc};

use auth::{
    hashing::{Hasher, initialize_hasher},
    token::{Token, access_token::AccessToken, refresh_token::RefreshToken},
    util,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;
use cookie::{
    Cookie, Expiration,
    time::{Duration, OffsetDateTime},
};
use serde_json::json;
use uuid::Uuid;

use crate::{model::user::LoginPayload, state::AppState};

pub async fn handler(
    State(state): State<Arc<AppState>>,
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

    let token_id = Uuid::new_v4().to_string();
    let at_lifetime: u64 = env::var("JWT_AT_LIFETIME").unwrap().parse().unwrap();
    let rt_lifetime: u64 = env::var("JWT_RT_LIFETIME").unwrap().parse().unwrap();
    let token_domain = env::var("COOKIE_DOMAIN").ok();
    let access_token = AccessToken::new(token_id.clone(), user_id.to_string(), at_lifetime);
    let refresh_token = RefreshToken::new(token_id, user_id.to_string(), rt_lifetime);

    let key = env::var("SECRET_JWT_SIGNING_KEY").unwrap();
    let key = util::key_from_slice(key.as_bytes()).unwrap();

    let access_token_string = access_token.sign_with_key(&key).unwrap();
    let refresh_token_string = refresh_token.sign_with_key(&key).unwrap();

    let mut at_cookie = Cookie::build(("at", access_token_string))
        .expires(Expiration::DateTime(
            OffsetDateTime::now_utc()
                .checked_add(Duration::seconds(at_lifetime as i64))
                .unwrap(),
        ))
        .http_only(true)
        .path("/")
        .build();
    let mut rt_cookie = Cookie::build(("rt", refresh_token_string))
        .expires(Expiration::DateTime(
            OffsetDateTime::now_utc()
                .checked_add(Duration::seconds(rt_lifetime as i64))
                .unwrap(),
        ))
        .http_only(true)
        .path("/auth/refresh")
        .build();
    if let Some(domain) = token_domain {
        at_cookie.set_domain(domain.clone());
        rt_cookie.set_domain(domain);
    }

    (StatusCode::OK, cookies.add(at_cookie).add(rt_cookie)).into_response()
}
