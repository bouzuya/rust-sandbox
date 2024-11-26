use std::str::FromStr as _;

use crate::session_id::SessionId;
use crate::user_id::UserId;
use crate::{session::Session, AppState};
use anyhow::Context as _;
use axum::{extract::State, routing::post, Json};

use super::{Claims, Error};

#[derive(serde::Deserialize)]
struct CreateSessionRequestBody {
    user_id: String,
    user_secret: String,
}

#[derive(serde::Serialize)]
struct CreateSessionResponse {
    session_token: String,
}

async fn create_session(
    State(app_state): State<AppState>,
    Json(CreateSessionRequestBody {
        user_id,
        user_secret,
    }): Json<CreateSessionRequestBody>,
) -> Result<Json<CreateSessionResponse>, Error> {
    let users = app_state.users.lock().await;
    let user_id = UserId::from_str(&user_id).map_err(|_| Error::Client)?;
    let user = users.get(&user_id).ok_or_else(|| Error::Client)?;
    user.secret
        .verify(&user_secret)
        .map_err(|_| Error::Client)?;

    let mut sessions = app_state.sessions.lock().await;
    let session_id = SessionId::generate();
    sessions.insert(
        session_id,
        Session {
            id: session_id,
            user_id,
            state: None,
        },
    );

    let encoding_key =
        jsonwebtoken::EncodingKey::from_rsa_pem(include_bytes!("../../private_key.pem"))
            .map_err(|_| Error::Server)?;
    let exp = std::time::SystemTime::now()
        .checked_add(std::time::Duration::from_secs(60 * 60))
        .context("std::time::SystemTime::checked_add")
        .map_err(|_| Error::Server)?
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .map_err(|_| Error::Server)?
        .as_secs();
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256),
        &Claims {
            exp,
            sid: session_id.to_string(),
            sub: user_id.to_string(),
        },
        &encoding_key,
    )
    .map_err(|_| Error::Server)?;
    Ok(Json(CreateSessionResponse {
        session_token: token.to_string(),
    }))
}

pub fn route() -> axum::Router<AppState> {
    axum::Router::new().route("/sessions", post(create_session))
}
