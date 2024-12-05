use crate::{session_id_extractor::SessionIdExtractor, AppState};
use axum::{
    extract::{Query, State},
    Json,
};

use super::Error;

#[derive(serde::Deserialize)]
struct CallbackQueryParams {
    // authuser: String,
    code: String,
    // hd: String,
    // prompt: String
    // scope: String,
    state: String,
}

async fn handle(
    SessionIdExtractor(session_id): SessionIdExtractor,
    Query(query): Query<CallbackQueryParams>,
    State(app_state): State<AppState>,
) -> Result<Json<String>, Error> {
    let mut sessions = app_state.sessions.lock().await;
    let session = sessions.get_mut(&session_id).ok_or_else(|| {
        Error::Client(anyhow::anyhow!(
            "associate_google_account session not found"
        ))
    })?;
    if session.state != Some(query.state) {
        return Err(Error::Client(anyhow::anyhow!(
            "associate_google_account session state not match"
        )));
    }

    let nonce = session
        .nonce
        .clone()
        .ok_or_else(|| Error::Client(anyhow::anyhow!("associate_google_account nonce is none")))?;
    // FIXME: Error::Client or Error::Server
    let google_account_id = app_state
        .send_token_request_and_verify_id_token(query.code, nonce)
        .await
        .map_err(Error::Server)?;
    session.nonce = None;

    let mut google_accounts = app_state.google_accounts.lock().await;
    if google_accounts.contains_key(&google_account_id) {
        return Err(Error::Client(anyhow::anyhow!(
            "associate_google_account already associated"
        )));
    }
    google_accounts
        .entry(google_account_id)
        .or_insert(session.user_id);

    // FIXME: fetch the user_id using the id token

    Ok(Json("OK".to_owned()))
}

pub fn route() -> axum::Router<AppState> {
    // FIXME: path
    axum::Router::new().route("/associate_google_account", axum::routing::post(handle))
}
