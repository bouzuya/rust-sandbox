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
    access_token: String,
    expires_in: u32,
    id_token: String,
    scope: String,
    token_type: String,
}

async fn handle(
    SessionIdExtractor(session_id): SessionIdExtractor,
    Query(query): Query<CallbackQueryParams>,
    State(app_state): State<AppState>,
) -> Result<Json<String>, Error> {
    let sessions = app_state.sessions.lock().await;
    let session = sessions.get(&session_id).ok_or_else(|| Error::Client)?;
    if session.state != Some(query.state) {
        return Err(Error::Client);
    }

    let redirect_uri = "http://localhost:3000/".to_owned();

    let response = reqwest::Client::new()
        .post(app_state.token_endpoint)
        .json(&TokenRequestBody {
            code: query.code,
            client_id: app_state.client_id,
            client_secret: app_state.client_secret,
            redirect_uri,
            grant_type: "authorization_code".to_owned(),
        })
        .send()
        .await
        .map_err(|_| Error::Server)?;
    if !response.status().is_success() {
        println!("status code = {}", response.status());
        println!(
            "response body = {}",
            response.text().await.map_err(|_| Error::Server)?
        );
        return Err(Error::Client);
    } else {
        let response_body = response.text().await.map_err(|_| Error::Server)?;
        println!("response body = {}", response_body);

        let token_response_body =
            serde_json::from_str::<TokenResponseBody>(&response_body).map_err(|_| Error::Server)?;
        println!("token response body = {:?}", token_response_body);

        // FIXME: cache jwks
        let response = reqwest::get(&app_state.jwks_uri)
            .await
            .map_err(|_| Error::Server)?;
        let jwks: jsonwebtoken::jwk::JwkSet = response.json().await.map_err(|_| Error::Server)?;

        let header = jsonwebtoken::decode_header(&token_response_body.id_token)
            .map_err(|_| Error::Server)?;
        let kid = header.kid.ok_or_else(|| Error::Server)?;
        let jwk = jwks.find(&kid).ok_or_else(|| Error::Server)?;
        let decoding_key = jsonwebtoken::DecodingKey::from_jwk(&jwk).map_err(|_| Error::Server)?;
        #[derive(serde::Deserialize)]
        struct IdTokenClaims {
            nonce: String,
        }
        let decoded = jsonwebtoken::decode::<IdTokenClaims>(
            &token_response_body.id_token,
            &decoding_key,
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256),
        )
        .map_err(|_| Error::Server)?;
        if Some(decoded.claims.nonce) != session.nonce {
            return Err(Error::Server);
        }

        // FIXME: fetch the user_id using the id token

        Ok(Json(response_body))
    }
}

pub fn route() -> axum::Router<AppState> {
    // FIXME: path
    axum::Router::new().route("/associate_google_account", axum::routing::post(handle))
}
