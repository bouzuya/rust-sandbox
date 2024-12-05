use crate::session_id_extractor::SessionIdExtractor;
use crate::AppState;
use axum::{extract::State, routing::post, Json};

use super::Error;

#[derive(serde::Deserialize)]
struct RequestBody {
    code: String,
    state: String,
}

#[derive(serde::Serialize)]
struct ResponseBody {
    session_token: String,
}

async fn handle(
    SessionIdExtractor(session_id): SessionIdExtractor,
    State(app_state): State<AppState>,
    Json(RequestBody { code, state }): Json<RequestBody>,
) -> Result<Json<ResponseBody>, Error> {
    let mut sessions = app_state.sessions.lock().await;
    let session = sessions
        .get_mut(&session_id)
        .ok_or_else(|| Error::Client(anyhow::anyhow!("sign_in session not found")))?;
    if session.state != Some(state) {
        return Err(Error::Client(anyhow::anyhow!(
            "sign_in session state not match"
        )));
    }

    let nonce = session
        .nonce
        .clone()
        .ok_or_else(|| Error::Client(anyhow::anyhow!("sign_in nonce not found")))?;
    // FIXME: Error::Client or Error::Server
    let google_account_id = app_state
        .send_token_request_and_verify_id_token(code, nonce)
        .await
        .map_err(Error::Server)?;
    session.nonce = None;

    let google_accounts = app_state.google_accounts.lock().await;
    let user_id = *google_accounts
        .get(&google_account_id)
        .ok_or_else(|| Error::Client(anyhow::anyhow!("sign_in user not found")))?;

    let session_token = app_state
        .create_session_token(user_id)
        .await
        .map_err(Error::Server)?;

    Ok(Json(ResponseBody { session_token }))
}

pub fn route() -> axum::Router<AppState> {
    // FIXME: path
    axum::Router::new().route("/sign_in", post(handle))
}
