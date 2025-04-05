use std::sync::Arc;

use auth::session::{data::SessionData, store::integration::axum::Session};
use axum::{
    extract::State,
    http::{Extensions, StatusCode},
    response::IntoResponse,
};
use serde_json::json;

use crate::{
    model::user::{User, UserPayload},
    state::AppState,
};

#[axum::debug_handler]
pub async fn handler(session: Session, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let user = state
        .db
        .query_unpaged(
            r#"SELECT * FROM ks.users WHERE id = ? LIMIT 1"#,
            (session.0.user_id,),
        )
        .await
        .unwrap();
    let user = user
        .into_rows_result()
        .unwrap()
        .first_row::<User>()
        .unwrap();
    let payload = UserPayload::from_user(&user);

    json!({ "data": payload }).to_string().into_response()
}
