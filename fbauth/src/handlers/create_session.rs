use std::str::FromStr as _;

use crate::session_id::SessionId;
use crate::user_id::UserId;
use crate::{session::Session, AppState};
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
    let users = app_state.users.lock()?;
    let user_id = UserId::from_str(&user_id).map_err(|_| Error::Client)?;
    let user = users.get(&user_id).ok_or_else(|| Error::Client)?;
    user.secret
        .verify(&user_secret)
        .map_err(|_| Error::Client)?;

    let mut sessions = app_state.sessions.lock().map_err(|_| Error::Server)?;
    let session_id = SessionId::generate();
    sessions.insert(
        session_id,
        Session {
            id: session_id,
            user_id,
            state: None,
        },
    );

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256),
        &Claims {
            sid: session_id.to_string(),
            sub: user_id.to_string(),
        },
        &jsonwebtoken::EncodingKey::from_rsa_pem(include_bytes!("../../key.pem"))
            .map_err(|_| Error::Server)?,
    )
    .map_err(|_| Error::Server)?;
    Ok(Json(CreateSessionResponse {
        session_token: token.to_string(),
    }))
}

pub fn route() -> axum::Router<AppState> {
    axum::Router::new().route("/sessions", post(create_session))
}
