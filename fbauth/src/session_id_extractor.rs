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
            .ok_or_else(|| Error::Client)?
            .to_str()
            .map_err(|_| Error::Client)?
            .strip_prefix("Bearer ")
            .ok_or_else(|| Error::Client)?;

        let decoding_key =
            jsonwebtoken::DecodingKey::from_rsa_pem(include_bytes!("../public_key.pem"))
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
