use crate::session_id_extractor::SessionIdExtractor;

use super::{AppState, Error};

use anyhow::Context as _;
use argon2::password_hash::rand_core::{OsRng, RngCore};
use axum::{extract::State, Json};

#[derive(serde::Serialize)]
struct ResponseBody {
    authorization_url: String,
}

impl std::fmt::Debug for ResponseBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResponseBody")
            .field("authorization_url", &"[FILTERED]")
            .finish()
    }
}

#[tracing::instrument(err(Debug), ret(level = tracing::Level::DEBUG), skip(app_state))]
async fn handle(
    SessionIdExtractor(session_id): SessionIdExtractor,
    State(app_state): State<AppState>,
) -> Result<Json<ResponseBody>, Error> {
    tracing::debug!("create authorization_url");
    let mut sessions = app_state.sessions.lock().await;

    // generate state
    let state = {
        let mut bytes = [0u8; 36];
        OsRng.fill_bytes(&mut bytes);
        hex::encode(bytes)
    };
    // generate nonce
    let nonce = {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        hex::encode(bytes)
    };

    // set state and nonce to session
    sessions.entry(session_id).and_modify(|session| {
        session.state = Some(state.to_owned());
        session.nonce = Some(nonce.to_owned());
    });

    let client_id = &app_state.client_id;

    let redirect_uri = "http://localhost:3000/";
    let mut url = url::Url::parse(&app_state.authorization_endpoint)
        .context("create_authorization_url Url::parse(authorization_endpoint)")
        .map_err(Error::Server)?;
    url.query_pairs_mut()
        .clear()
        .append_pair("client_id", client_id)
        .append_pair("nonce", &nonce)
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("response_type", "code")
        .append_pair("scope", "openid email")
        .append_pair("state", &state);
    let authorization_url = url.to_string();
    tracing::debug!("authorization_url created");
    Ok(Json(ResponseBody { authorization_url }))
}

pub fn route() -> axum::Router<AppState> {
    axum::Router::new().route("/authorization_urls", axum::routing::post(handle))
}
