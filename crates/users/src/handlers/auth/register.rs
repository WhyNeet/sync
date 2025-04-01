use std::sync::Arc;

use argon2::{
    Argon2, Params,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{Json, extract::State, response::IntoResponse};
use serde_json::json;
use uuid::Uuid;

use crate::{
    model::user::{CreateUserPayload, User, UserPayload},
    state::AppState,
};

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    if payload.password.len() < 8 {
        return json!({ "kind": "error", "data": "Password must contain at least 8 characters." })
            .to_string();
    }

    let username_check = state
        .db
        .query_unpaged(
            r#"SELECT count(*) FROM ks.users_by_username WHERE username = ? LIMIT 1"#,
            (payload.username.as_str(),),
        )
        .await
        .unwrap();
    let username_check = username_check
        .into_rows_result()
        .unwrap()
        .first_row::<(i64,)>()
        .unwrap();
    if username_check.0 != 0 {
        return json!({ "kind": "error", "data": "Username already exists." }).to_string();
    }

    let user = User {
        id: Uuid::new_v4(),
        avatar_id: Uuid::new_v4(),
        display_name: payload.display_name,
        email: payload.email,
        username: payload.username,
        password: {
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::new(
                argon2::Algorithm::Argon2id,
                argon2::Version::V0x13,
                Params::new(7168, 5, 1, None).unwrap(),
            );
            argon2
                .hash_password(payload.password.as_bytes(), &salt)
                .unwrap()
                .to_string()
        },
    };

    let payload = UserPayload::from_user(&user);

    state
        .db
        .query_unpaged(
            r#"INSERT INTO ks.users_by_username (id, username) VALUES (?, ?)"#,
            (&user.id, &user.username),
        )
        .await
        .unwrap();
    state.db.query_unpaged(r#"INSERT INTO ks.users (id, email, avatar_id, display_name, password, username) VALUES (?, ?, ?, ?, ?, ?)"#, user).await.unwrap();

    json!({ "data": payload }).to_string()
}
