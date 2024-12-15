use crate::{
    google_account_id::GoogleAccountId, session_id_extractor::SessionIdExtractor, user::User,
    AppState,
};
use anyhow::Context as _;
use axum::{extract::State, Json};

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
    user_id: String,
}

impl std::fmt::Debug for ResponseBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResponseBody")
            .field("session_token", &"[FILTERED]")
            .field("user_id", &self.user_id)
            .finish()
    }
}

#[tracing::instrument(err(Debug), ret(level = tracing::Level::DEBUG), skip(app_state))]
async fn handle(
    SessionIdExtractor(session_id): SessionIdExtractor,
    State(app_state): State<AppState>,
    Json(body): Json<RequestBody>,
) -> Result<Json<ResponseBody>, Error> {
    tracing::debug!("sign up");
    let user_id = {
        let mut sessions = app_state.sessions.lock().await;
        let session = sessions
            .get_mut(&session_id)
            .ok_or_else(|| Error::Client(anyhow::anyhow!("sign_up session not found")))?;
        if session.state != Some(body.state) {
            return Err(Error::Client(anyhow::anyhow!(
                "sign_up session state not match"
            )));
        }

        let nonce = session.nonce.clone().ok_or_else(|| {
            Error::Client(anyhow::anyhow!("associate_google_account nonce is none"))
        })?;
        // FIXME: Error::Client or Error::Server
        let google_account_id = app_state
            .send_token_request_and_verify_id_token(body.code, nonce)
            .await
            .map_err(Error::Server)?;
        let google_account_id =
            GoogleAccountId::try_from(google_account_id).map_err(Error::Server)?;
        session.nonce = None;

        // FIXME: fetch the user_id using the id token

        let mut user_store = app_state.user_store.lock().await;
        let user = User::new()
            .context("create_user User::new")
            .map_err(Error::Server)?;
        user_store.users.insert(user.id, user.clone());

        if user_store.google_accounts.contains_key(&google_account_id) {
            return Err(Error::Client(anyhow::anyhow!(
                "associate_google_account already associated"
            )));
        }
        user_store
            .google_accounts
            .entry(google_account_id)
            .or_insert(user.id);

        session.user_id = Some(user.id);

        user.id
    };

    let session_token = app_state
        .create_session_token(user_id)
        .await
        .map_err(Error::Server)?;

    tracing::debug!("signed up");

    Ok(Json(ResponseBody {
        session_token,
        user_id: user_id.to_string(),
    }))
}

pub fn route() -> axum::Router<AppState> {
    // FIXME: path
    axum::Router::new().route("/sign_up", axum::routing::post(handle))
}
