use std::str::FromStr as _;

use crate::user_id::UserId;
use crate::AppState;
use anyhow::Context;
use axum::{extract::State, routing::post, Json};

use super::Error;

#[derive(serde::Deserialize)]
struct RequestBody {
    user_id: String,
    user_secret: String,
}

#[derive(serde::Serialize)]
struct ResponseBody {
    session_token: String,
}

async fn handle(
    State(app_state): State<AppState>,
    Json(RequestBody {
        user_id,
        user_secret,
    }): Json<RequestBody>,
) -> Result<Json<ResponseBody>, Error> {
    let users = app_state.users.lock().await;
    let user_id = UserId::from_str(&user_id)
        .context("create_session UserId::from_str")
        .map_err(Error::Client)?;
    let user = users
        .get(&user_id)
        .ok_or_else(|| Error::Client(anyhow::anyhow!("create_session user not found")))?;
    user.secret
        .verify(&user_secret)
        .context("create_session UserSecret::verify")
        .map_err(Error::Client)?;

    let session_token = app_state
        .create_session_token(user_id)
        .await
        .map_err(Error::Server)?;

    Ok(Json(ResponseBody { session_token }))
}

pub fn route() -> axum::Router<AppState> {
    axum::Router::new().route("/sessions", post(handle))
}
