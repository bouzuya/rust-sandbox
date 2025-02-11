use std::{collections::HashMap, sync::Arc};

use anyhow::Context as _;
use tokio::sync::Mutex;

use crate::discovery_document::DiscoveryDocument;
use crate::handlers::Claims;
use crate::models::google_account_id::GoogleAccountId;
use crate::models::session::Session;
use crate::models::session_id::SessionId;
use crate::models::user::User;
use crate::models::user_id::UserId;

/// CREATE TABLE users (
///   id             VARCHAR NOT NULL,
///   google_account VARCHAR     NULL,
///   PRIMARY KEY (id),
///   UNIQUE (google_account),
/// );
#[derive(Clone, Debug, Default)]
pub(crate) struct UserStore {
    pub(crate) google_account_id_index: HashMap<GoogleAccountId, UserId>,
    pub(crate) users: HashMap<UserId, User>,
}

impl UserStore {
    pub(crate) fn create(&mut self, user: User) -> anyhow::Result<()> {
        if self
            .google_account_id_index
            .contains_key(&user.google_account_id)
        {
            anyhow::bail!("google_account_id is already in use")
        }

        if self.users.contains_key(&user.id) {
            anyhow::bail!("user_id is already in use")
        }

        self.google_account_id_index
            .insert(user.google_account_id.clone(), user.id);
        self.users.insert(user.id, user);

        Ok(())
    }

    pub(crate) fn find_by_google_account(
        &self,
        google_account_id: &GoogleAccountId,
    ) -> Option<User> {
        match self.google_account_id_index.get(google_account_id) {
            None => None,
            Some(user_id) => self.users.get(&user_id).cloned(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) authorization_endpoint: String,
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) decoding_key: jsonwebtoken::DecodingKey,
    pub(crate) encoding_key: jsonwebtoken::EncodingKey,
    pub(crate) jwks_uri: String,
    pub(crate) sessions: Arc<Mutex<HashMap<SessionId, Session>>>,
    pub(crate) token_endpoint: String,
    pub(crate) user_store: Arc<Mutex<UserStore>>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let decoding_key =
            jsonwebtoken::DecodingKey::from_rsa_pem(include_bytes!("../public_key.pem"))
                .context("DecodingKey::from_rsa_pem")?;
        let encoding_key =
            jsonwebtoken::EncodingKey::from_rsa_pem(include_bytes!("../private_key.pem"))
                .context("EncodingKey::from_rsa_pem")?;

        let DiscoveryDocument {
            authorization_endpoint,
            jwks_uri,
            token_endpoint,
        } = DiscoveryDocument::fetch().await?;
        let client_id = std::env::var("CLIENT_ID")?;
        let client_secret = std::env::var("CLIENT_SECRET")?;

        tracing::debug!(
            authorization_endpoint,
            client_id,
            client_secret,
            token_endpoint,
            "config loaded"
        );

        Ok(Self {
            authorization_endpoint,
            client_id,
            client_secret,
            decoding_key,
            encoding_key,
            jwks_uri,
            sessions: Arc::new(Mutex::new(Default::default())),
            token_endpoint,
            user_store: Arc::new(Mutex::new(UserStore::default())),
        })
    }
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState")
            .field("authorization_endpoint", &self.authorization_endpoint)
            .field("client_id", &self.client_id)
            .field("client_secret", &"[FILTERED]")
            .field("encoding_key", &"[FILTERED]")
            .field("jwks_uri", &self.jwks_uri)
            .field("sessions", &self.sessions)
            .field("token_endpoint", &self.token_endpoint)
            .field("user_store", &self.user_store)
            .finish()
    }
}

impl AppState {
    pub async fn create_anonymous_session(&self) -> anyhow::Result<String> {
        let mut sessions = self.sessions.lock().await;
        let session_id = SessionId::generate();
        sessions.insert(
            session_id,
            Session {
                id: session_id,
                nonce: None,
                user_id: None,
                state: None,
            },
        );
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
                sub: None,
            },
            &self.encoding_key,
        )
        .context("create_session encode")?;
        Ok(token.to_string())
    }

    pub async fn create_session_token(&self, user_id: UserId) -> anyhow::Result<String> {
        let mut sessions = self.sessions.lock().await;
        let session_id = SessionId::generate();
        sessions.insert(
            session_id,
            Session {
                id: session_id,
                nonce: None,
                user_id: Some(user_id),
                state: None,
            },
        );

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
                sub: Some(user_id.to_string()),
            },
            &self.encoding_key,
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
            client_id: String,
            client_secret: String,
            code: String,
            grant_type: String,
            redirect_uri: String,
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
                client_id: self.client_id.clone(),
                client_secret: self.client_secret.clone(),
                code,
                grant_type: "authorization_code".to_owned(),
                redirect_uri,
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
