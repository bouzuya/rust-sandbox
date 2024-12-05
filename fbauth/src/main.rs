mod discovery_document;
mod handlers;
mod session;
mod session_id;
mod session_id_extractor;
mod user;
mod user_id;
mod user_secret;

use std::{collections::HashMap, sync::Arc};

use anyhow::Context as _;
use handlers::Claims;
use tokio::sync::Mutex;

use discovery_document::DiscoveryDocument;
use session::Session;
use session_id::SessionId;
use tower_http::trace::TraceLayer;
use user::User;
use user_id::UserId;

#[derive(Clone)]
pub(crate) struct AppState {
    authorization_endpoint: String,
    client_id: String,
    client_secret: String,
    google_accounts: Arc<Mutex<HashMap<String, UserId>>>,
    jwks_uri: String,
    sessions: Arc<Mutex<HashMap<SessionId, Session>>>,
    token_endpoint: String,
    users: Arc<Mutex<HashMap<UserId, User>>>,
}

impl AppState {
    pub async fn create_session_token(&self, user_id: UserId) -> anyhow::Result<String> {
        let mut sessions = self.sessions.lock().await;
        let session_id = SessionId::generate();
        sessions.insert(
            session_id,
            Session {
                id: session_id,
                nonce: None,
                user_id,
                state: None,
            },
        );

        let encoding_key =
            jsonwebtoken::EncodingKey::from_rsa_pem(include_bytes!("../private_key.pem"))
                .context("create_session EncodingKey::from_rsa_pem")?;
        let exp = std::time::SystemTime::now()
            .checked_add(std::time::Duration::from_secs(60 * 60))
            .context("create_session std::time::SystemTime::checked_add")?
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .context("create_session SystemTime::duration_since")?
            .as_secs();
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256),
            &Claims {
                exp,
                sid: session_id.to_string(),
                sub: user_id.to_string(),
            },
            &encoding_key,
        )
        .context("create_session encode")?;
        Ok(token.to_string())
    }

    pub async fn send_token_request_and_verify_id_token(
        &self,
        code: String,
        nonce: String,
    ) -> anyhow::Result<String> {
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

        let redirect_uri = "http://localhost:3000/".to_owned();

        let response = reqwest::Client::new()
            .post(self.token_endpoint.clone())
            .json(&TokenRequestBody {
                code,
                client_id: self.client_id.clone(),
                client_secret: self.client_secret.clone(),
                redirect_uri,
                grant_type: "authorization_code".to_owned(),
            })
            .send()
            .await
            .context("associate_google_account token request")?;
        if !response.status().is_success() {
            tracing::error!(
                "token request status is not success status_code = {}",
                response.status()
            );
            let response_body = response
                .text()
                .await
                .context("associate_google_account response.text")?;
            tracing::error!("token request response body = {}", response_body);
            return Err(anyhow::anyhow!(
                "associate_google_account status is not success"
            ));
        } else {
            let response_body = response
                .text()
                .await
                .context("associate_google_account response.text")?;
            tracing::debug!("token request response body = {}", response_body);

            let response_body = serde_json::from_str::<TokenResponseBody>(&response_body)
                .context("associate_google_account serde_json::from_str")?;
            tracing::debug!("token request response body (parsed) = {:?}", response_body);

            // FIXME: cache jwks
            let response = reqwest::get(&self.jwks_uri)
                .await
                .context("associate_google_account request::get(jwks_uri)")?;
            tracing::debug!("fetched jwks = {:?}", response.status());
            let jwks: jsonwebtoken::jwk::JwkSet = response
                .json()
                .await
                .context("associate_google_account response.json (JwkSet)")?;
            tracing::debug!("parsed jwks = {:?}", jwks);

            let header = jsonwebtoken::decode_header(&response_body.id_token)
                .context("associate_google_account decode_header")?;
            tracing::debug!("decode_header = {:?}", header);
            let kid = header
                .kid
                .ok_or_else(|| anyhow::anyhow!("associate_google_account kid not found"))?;
            tracing::debug!("kid = {:?}", kid);
            let jwk = jwks
                .find(&kid)
                .ok_or_else(|| anyhow::anyhow!("associate_google_account jwk not found"))?;
            tracing::debug!("jwk = {:?}", jwk);
            let decoding_key = jsonwebtoken::DecodingKey::from_jwk(&jwk)
                .context("associate_google_account DecodingKey::from_jwk")?;

            #[derive(Debug, serde::Deserialize)]
            struct IdTokenClaims {
                nonce: String,
                sub: String,
            }
            let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
            validation.set_audience(&[self.client_id.clone()]);
            validation.set_issuer(&["https://accounts.google.com"]);
            let decoded = jsonwebtoken::decode::<IdTokenClaims>(
                &response_body.id_token,
                &decoding_key,
                &validation,
            )
            .context("associate_google_account jsonwebtoken::decode")?;
            tracing::debug!("decoded = {:?}", decoded);
            if decoded.claims.nonce != nonce {
                return Err(anyhow::anyhow!("associate_google_account nonce not match"));
            }

            Ok(decoded.claims.sub)
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let DiscoveryDocument {
        authorization_endpoint,
        jwks_uri,
        token_endpoint,
    } = DiscoveryDocument::fetch().await?;
    let client_id = std::env::var("CLIENT_ID")?;
    let client_secret = std::env::var("CLIENT_SECRET")?;

    tracing::debug!("authorization_endpoint={}", authorization_endpoint);
    tracing::debug!("client_id={}", client_id);
    tracing::debug!("client_secret={}", client_secret);
    tracing::debug!("token_endpoint={}", token_endpoint);

    let state = AppState {
        authorization_endpoint,
        client_id,
        client_secret,
        google_accounts: Arc::new(Mutex::new(Default::default())),
        jwks_uri,
        sessions: Arc::new(Mutex::new(Default::default())),
        token_endpoint,
        users: Arc::new(Mutex::new(Default::default())),
    };
    let router = handlers::route()
        .with_state(state)
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, router).await?;
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() -> anyhow::Result<()> {
        let url = url::Url::parse("https://accounts.google.com/o/oauth2/v2/auth")?;
        assert_eq!(
            url.to_string(),
            "https://accounts.google.com/o/oauth2/v2/auth"
        );
        Ok(())
    }

    #[test]
    fn test_jwt() -> anyhow::Result<()> {
        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        pub(crate) struct Claims {
            /// session_id
            exp: u64,
            sid: String,
            sub: String,
            // ...
        }

        let exp = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_secs()
            + 60 * 60;

        let pem = include_bytes!("../private_key.pem");
        let encoding_key = jsonwebtoken::EncodingKey::from_rsa_pem(pem)?;
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256),
            &Claims {
                exp,
                sid: "sid1".to_owned(),
                sub: "uid1".to_owned(),
            },
            &encoding_key,
        )?;
        let pem = include_bytes!("../public_key.pem");
        let decoding_key = jsonwebtoken::DecodingKey::from_rsa_pem(pem)?;
        let jsonwebtoken::TokenData { header: _, claims } = jsonwebtoken::decode::<Claims>(
            &token,
            &decoding_key,
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256),
        )?;

        assert_ne!(format!("{:?}", claims), "");

        Ok(())
    }
}
