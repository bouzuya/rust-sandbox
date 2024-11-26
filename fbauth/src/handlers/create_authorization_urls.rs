use crate::session_id_extractor::SessionIdExtractor;

use super::{AppState, Error};

use argon2::password_hash::rand_core::{OsRng, RngCore};
use axum::{extract::State, Json};

#[derive(serde::Serialize)]
struct CreateAuthorizationUrlResponseBody {
    authorization_url: String,
}

async fn create_authorization_url(
    SessionIdExtractor(session_id): SessionIdExtractor,
    State(app_state): State<AppState>,
) -> Result<Json<CreateAuthorizationUrlResponseBody>, Error> {
    let mut sessions = app_state.sessions.lock().await;

    // generate state
    let state = {
        let mut bytes = [0u8; 36];
        OsRng.fill_bytes(&mut bytes);
        hex::encode(bytes)
    };

    // set state to session
    sessions.entry(session_id).and_modify(|session| {
        session.state = Some(state.to_owned());
    });

    let client_id = &app_state.client_id;
    let nonce = "FIXME";
    let redirect_uri = "http://localhost:3000/callback";
    let mut url = url::Url::parse(&app_state.authorization_endpoint).map_err(|_| Error::Server)?;
    url.query_pairs_mut()
        .clear()
        .append_pair("response_type", "code")
        .append_pair("client_id", client_id)
        .append_pair("scope", "openid email")
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("state", &state)
        .append_pair("nonce", nonce);
    let authorization_url = url.to_string();
    Ok(Json(CreateAuthorizationUrlResponseBody {
        authorization_url,
    }))
}

pub fn route() -> axum::Router<AppState> {
    axum::Router::new().route(
        "/authorization_urls",
        axum::routing::post(create_authorization_url),
    )
}
