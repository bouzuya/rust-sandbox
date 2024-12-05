use crate::{session_id_extractor::SessionIdExtractor, AppState};
use anyhow::Context as _;
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

#[derive(serde::Serialize)]
struct TokenRequestBody {
    code: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    grant_type: String,
}

#[derive(Debug, serde::Deserialize)]
struct TokenResponseBody {
    // access_token: String,
    // expires_in: u32,
    id_token: String,
    // scope: String,
    // token_type: String,
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

    let redirect_uri = "http://localhost:3000/".to_owned();

    let response = reqwest::Client::new()
        .post(app_state.token_endpoint)
        .json(&TokenRequestBody {
            code: query.code,
            client_id: app_state.client_id.clone(),
            client_secret: app_state.client_secret,
            redirect_uri,
            grant_type: "authorization_code".to_owned(),
        })
        .send()
        .await
        .context("associate_google_account token request")
        .map_err(Error::Server)?;
    if !response.status().is_success() {
        tracing::error!(
            "token request status is not success status_code = {}",
            response.status()
        );
        let response_body = response
            .text()
            .await
            .context("associate_google_account response.text")
            .map_err(Error::Server)?;
        tracing::error!("token request response body = {}", response_body);
        return Err(Error::Client(anyhow::anyhow!(
            "associate_google_account status is not success"
        )));
    } else {
        let response_body = response
            .text()
            .await
            .context("associate_google_account response.text")
            .map_err(Error::Server)?;
        tracing::debug!("token request response body = {}", response_body);

        let response_body = serde_json::from_str::<TokenResponseBody>(&response_body)
            .context("associate_google_account serde_json::from_str")
            .map_err(Error::Server)?;
        tracing::debug!("token request response body (parsed) = {:?}", response_body);

        // FIXME: cache jwks
        let response = reqwest::get(&app_state.jwks_uri)
            .await
            .context("associate_google_account request::get(jwks_uri)")
            .map_err(Error::Server)?;
        tracing::debug!("fetched jwks = {:?}", response.status());
        let jwks: jsonwebtoken::jwk::JwkSet = response
            .json()
            .await
            .context("associate_google_account response.json (JwkSet)")
            .map_err(Error::Server)?;
        tracing::debug!("parsed jwks = {:?}", jwks);

        let header = jsonwebtoken::decode_header(&response_body.id_token)
            .context("associate_google_account decode_header")
            .map_err(Error::Server)?;
        tracing::debug!("decode_header = {:?}", header);
        let kid = header.kid.ok_or_else(|| {
            Error::Server(anyhow::anyhow!("associate_google_account kid not found"))
        })?;
        tracing::debug!("kid = {:?}", kid);
        let jwk = jwks.find(&kid).ok_or_else(|| {
            Error::Server(anyhow::anyhow!("associate_google_account jwk not found"))
        })?;
        tracing::debug!("jwk = {:?}", jwk);
        let decoding_key = jsonwebtoken::DecodingKey::from_jwk(&jwk)
            .context("associate_google_account DecodingKey::from_jwk")
            .map_err(Error::Server)?;
        #[derive(Debug, serde::Deserialize)]
        struct IdTokenClaims {
            nonce: String,
            sub: String,
        }
        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.set_audience(&[app_state.client_id]);
        validation.set_issuer(&["https://accounts.google.com"]);
        let decoded = jsonwebtoken::decode::<IdTokenClaims>(
            &response_body.id_token,
            &decoding_key,
            &validation,
        )
        .context("associate_google_account jsonwebtoken::decode")
        .map_err(Error::Server)?;
        tracing::debug!("decoded = {:?}", decoded);
        if Some(decoded.claims.nonce) != session.nonce {
            return Err(Error::Server(anyhow::anyhow!(
                "associate_google_account nonce not match"
            )));
        }

        session.nonce = None;

        let mut google_accounts = app_state.google_accounts.lock().await;
        if google_accounts.contains_key(&decoded.claims.sub) {
            return Err(Error::Client(anyhow::anyhow!(
                "associate_google_account already associated"
            )));
        }
        google_accounts
            .entry(decoded.claims.sub)
            .or_insert(session.user_id);

        // FIXME: fetch the user_id using the id token

        Ok(Json("OK".to_owned()))
    }
}

pub fn route() -> axum::Router<AppState> {
    // FIXME: path
    axum::Router::new().route("/associate_google_account", axum::routing::post(handle))
}
