use std::{fmt::Display, str::FromStr};

use anyhow::Context;
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};

use crate::twitter::AccessTokenResponse;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Token {
    pub access_token: String,
    pub expires: String,
    pub refresh_token: String,
}

impl FromStr for Token {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(s)?)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).expect("to_string"))
    }
}

impl Token {
    pub fn try_from(
        access_token_response: AccessTokenResponse,
        unix_timestamp: i64,
    ) -> anyhow::Result<Token> {
        let now = OffsetDateTime::from_unix_timestamp(unix_timestamp)?;

        let access_token = access_token_response.access_token;
        let expires_in = access_token_response
            .expires_in
            .context("expires_in is none")?;
        let refresh_token = access_token_response
            .refresh_token
            .context("refresh_token is none")?;

        let expires = now + Duration::seconds(i64::from(expires_in));
        let expires = expires.format(&Rfc3339)?;
        Ok(Token {
            access_token,
            expires,
            refresh_token,
        })
    }
}
