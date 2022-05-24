use serde::{Deserialize, Serialize};

use crate::{post, Error};

#[derive(Debug, Serialize)]
pub struct AccessTokenRequest<'a> {
    pub consumer_key: &'a str,
    pub code: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub username: String,
    pub state: Option<String>,
}

pub async fn access_token_request(
    request: &AccessTokenRequest<'_>,
) -> Result<AccessTokenResponse, Error> {
    post("https://getpocket.com/v3/oauth/authorize", request).await
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        // TODO
    }
}
