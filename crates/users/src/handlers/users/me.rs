use std::{str::FromStr, sync::Arc};

use auth::session::store::integration::axum::SessionId;
use axum::{
    extract::State,
    http::{Extensions, StatusCode},
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    model::user::{User, UserPayload},
    state::AppState,
};

#[axum::debug_handler]
pub async fn handler(ext: Extensions, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let session = ext.get::<Option<SessionId>>().unwrap().as_ref().unwrap();

    let user_id = state
        .db
        .query_unpaged(
            "SELECT user_id FROM ks.sessions WHERE id = ? LIMIT 1",
            (Uuid::from_str(session).unwrap(),),
        )
        .await
        .unwrap()
        .into_rows_result()
        .unwrap()
        .first_row::<(Uuid,)>()
        .unwrap()
        .0;

    let user = state
        .db
        .query_unpaged(r#"SELECT * FROM ks.users WHERE id = ? LIMIT 1"#, (user_id,))
        .await
        .unwrap();
    let user = user
        .into_rows_result()
        .unwrap()
        .first_row::<User>()
        .unwrap();
    let payload = UserPayload::from_user(&user);

    return json!({ "data": payload }).to_string();
}
