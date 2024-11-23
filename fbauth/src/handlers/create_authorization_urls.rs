use super::{AppState, Claims, Error};

use std::str::FromStr as _;

use crate::session_id::SessionId;
use axum::{extract::State, Json};

struct SessionIdExtractor(SessionId);

#[axum::async_trait]
impl axum::extract::FromRequestParts<AppState> for SessionIdExtractor {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jwt = parts
            .headers
            .get("authorization")
            .ok_or_else(|| Error::Client)?
            .to_str()
            .map_err(|_| Error::Client)?
            .strip_prefix("Bearer ")
            .ok_or_else(|| Error::Client)?;

        let decoding_key = jsonwebtoken::DecodingKey::from_rsa_pem(include_bytes!("../../key.pem"))
            .map_err(|_| Error::Server)?;
        let jsonwebtoken::TokenData { header: _, claims } = jsonwebtoken::decode::<Claims>(
            jwt,
            &decoding_key,
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256),
        )
        .map_err(|_| Error::Client)?;

        let session_id = SessionId::from_str(&claims.sid).map_err(|_| Error::Client)?;
        Ok(Self(session_id))
    }
}

#[derive(serde::Serialize)]
struct CreateAuthorizationUrlResponseBody {
    authorization_url: String,
}

async fn create_authorization_url(
    SessionIdExtractor(session_id): SessionIdExtractor,
    State(app_state): State<AppState>,
) -> Result<Json<CreateAuthorizationUrlResponseBody>, Error> {
    let mut sessions = app_state.sessions.lock()?;

    // generate state
    let state = "FIXME";

    // set state to session
    sessions.entry(session_id).and_modify(|session| {
        session.state = Some(state.to_owned());
    });

    let client_id = &app_state.client_id;
    let nonce = "FIXME";
    let redirect_uri = "http://localhost:3000/callback";
    let state = "FIXME";
    let mut url = url::Url::parse(&app_state.authorization_endpoint).map_err(|_| Error::Server)?;
    url.query_pairs_mut()
        .clear()
        .append_pair("response_type", "code")
        .append_pair("client_id", client_id)
        .append_pair("scope", "openid email")
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("state", state)
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
