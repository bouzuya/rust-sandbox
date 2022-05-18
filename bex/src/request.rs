use std::collections::HashMap;

use hyper::StatusCode;
use reqwest::Response;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("request {0}")]
    Request(#[from] reqwest::Error),
    #[error(
        "status X-Error: {x_error:?}, X-Error-Code: {x_error_code:?}, HTTP Status: {status_code}"
    )]
    Status {
        status_code: u16,
        x_error_code: Option<String>,
        x_error: Option<String>,
    },
}

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

#[derive(Debug, Serialize)]
pub struct RetrieveRequest<'a> {
    pub consumer_key: &'a str,
    pub access_token: &'a str,
    pub state: Option<RetrieveRequestState>,
    pub count: Option<usize>,
    #[serde(rename = "detailType")]
    pub detail_type: Option<RetrieveRequestDetailType>,
    // ...
}

#[derive(Debug, Serialize)]
pub enum RetrieveRequestState {
    #[serde(rename = "unread")]
    Unread,
    #[serde(rename = "archive")]
    Archive,
    #[serde(rename = "all")]
    All,
}

#[derive(Debug, Serialize)]
pub enum RetrieveRequestDetailType {
    #[serde(rename = "simple")]
    Simple,
    #[serde(rename = "complete")]
    Complete,
}

pub type RetrieveResponse = HashMap<String, Value>;

// <https://getpocket.com/developer/docs/v3/retrieve>
pub async fn retrieve_request(request: &RetrieveRequest<'_>) -> Result<RetrieveResponse, Error> {
    post("https://getpocket.com/v3/get", request).await
}

fn check_status_code(response: &Response) -> Option<Error> {
    let status = response.status();
    if status == StatusCode::OK {
        return None;
    }

    let headers = response.headers();
    let x_error_code = headers.get("X-Error-Code");
    let x_error = headers.get("X-Error");
    Some(Error::Status {
        status_code: status.as_u16(),
        x_error_code: x_error_code
            .map(|v| v.to_str())
            .transpose()
            .unwrap()
            .map(|v| v.to_owned()),
        x_error: x_error
            .map(|v| v.to_str())
            .transpose()
            .unwrap()
            .map(|v| v.to_owned()),
    })
}

async fn post<T, U>(url: &str, body: &T) -> Result<U, Error>
where
    T: Serialize + ?Sized,
    U: DeserializeOwned,
{
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json; charset=UTF8")
        .header("X-Accept", "application/json")
        .json(&body)
        .send()
        .await?;
    if let Some(error) = check_status_code(&response) {
        return Err(error);
    }
    let response_body = response.json::<U>().await?;
    Ok(response_body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let request = RetrieveRequest {
            consumer_key: "consumer_key1",
            access_token: "access_token1",
            state: Some(RetrieveRequestState::Unread),
            count: Some(123),
            detail_type: Some(RetrieveRequestDetailType::Simple),
        };
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "state": "unread",
  "count": 123,
  "detailType": "simple"
}"#
        );
        Ok(())
    }

    #[test]
    fn retrieve_request_state() -> anyhow::Result<()> {
        let mut request = build_retrieve_request("consumer_key1", "access_token1");
        request.state = Some(RetrieveRequestState::Unread);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "state": "unread"
}"#
        );

        request.state = Some(RetrieveRequestState::Archive);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "state": "archive"
}"#
        );

        request.state = Some(RetrieveRequestState::All);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "state": "all"
}"#
        );
        Ok(())
    }

    fn build_retrieve_request<'a>(
        consumer_key: &'a str,
        access_token: &'a str,
    ) -> RetrieveRequest<'a> {
        RetrieveRequest {
            consumer_key,
            access_token,
            state: None,
            count: None,
            detail_type: None,
        }
    }
}
