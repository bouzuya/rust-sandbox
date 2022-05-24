use serde::{Deserialize, Serialize};

use crate::{post, Error};

#[derive(Debug, Serialize)]
pub struct AuthorizationRequest<'a> {
    pub consumer_key: &'a str,
    pub redirect_uri: &'a str,
    pub state: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
pub struct AuthorizationResponse {
    pub code: String,
}

pub async fn authorization_request(
    request: &AuthorizationRequest<'_>,
) -> Result<AuthorizationResponse, Error> {
    post("https://getpocket.com/v3/oauth/request", request).await
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        // TODO
    }
}
