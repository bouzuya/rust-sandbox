use std::str::FromStr as _;

use crate::AppState;
use crate::{google_account_id::GoogleAccountId, session_id_extractor::SessionIdExtractor};
use axum::{extract::State, routing::post, Json};

use super::Error;

#[derive(serde::Deserialize)]
struct RequestBody {
    code: String,
    state: String,
}

impl std::fmt::Debug for RequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RequestBody")
            .field("code", &"[FILTERED]")
            .field("state", &"[FILTERED]")
            .finish()
    }
}

#[derive(serde::Serialize)]
struct ResponseBody {
    session_token: String,
}

impl std::fmt::Debug for ResponseBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResponseBody")
            .field("session_token", &"[FILTERED]")
            .finish()
    }
}

#[tracing::instrument(err(Debug), ret(level = tracing::Level::DEBUG), skip(app_state))]
async fn handle(
    SessionIdExtractor(session_id): SessionIdExtractor,
    State(app_state): State<AppState>,
    Json(RequestBody { code, state }): Json<RequestBody>,
) -> Result<Json<ResponseBody>, Error> {
    tracing::debug!("sign in");
    let google_account_id = {
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

        GoogleAccountId::from_str(&google_account_id).map_err(Error::Server)?
    };

    let user_store = app_state.user_store.lock().await;
    let user_id = user_store
        .find_by_google_account(&google_account_id)
        .ok_or_else(|| Error::Client(anyhow::anyhow!("sign_in user not found")))?
        .id;

    let session_token = app_state
        .create_session_token(user_id)
        .await
        .map_err(Error::Server)?;
    tracing::debug!("signed in");

    Ok(Json(ResponseBody { session_token }))
}

pub fn route() -> axum::Router<AppState> {
    // FIXME: path
    axum::Router::new().route("/sign_in", post(handle))
}
