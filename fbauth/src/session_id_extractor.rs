use anyhow::Context;

use super::AppState;

use std::str::FromStr as _;

use crate::handlers::{Claims, Error};
use crate::session_id::SessionId;

pub(crate) struct SessionIdExtractor(pub(crate) SessionId);

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
            .ok_or_else(|| {
                Error::Client(anyhow::anyhow!(
                    "SessionIdExtractor authorization header not found"
                ))
            })?
            .to_str()
            .context("SessionIdExtractor authorization header value is not UTF-8")
            .map_err(Error::Client)?
            .strip_prefix("Bearer ")
            .ok_or_else(|| {
                Error::Client(anyhow::anyhow!(
                    "SessionIdExtractor authorization header value does not start with `Bearer `"
                ))
            })?;

        let decoding_key =
            jsonwebtoken::DecodingKey::from_rsa_pem(include_bytes!("../public_key.pem"))
                .context("SessionIdExtractor DecodingKey::from_rsa_pem")
                .map_err(Error::Server)?;
        let jsonwebtoken::TokenData { header: _, claims } = jsonwebtoken::decode::<Claims>(
            jwt,
            &decoding_key,
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256),
        )
        .context("SessionIdExtractor decode")
        .map_err(Error::Client)?;

        let session_id = SessionId::from_str(&claims.sid)
            .context("SessionIdExtractor SessionId::from_str")
            .map_err(Error::Client)?;
        Ok(Self(session_id))
    }
}
