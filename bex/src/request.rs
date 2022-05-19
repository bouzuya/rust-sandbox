use std::collections::HashMap;

use hyper::StatusCode;
use reqwest::Response;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use serde_repr::Serialize_repr;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<RetrieveRequestState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite: Option<RetrieveRequestFavorite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<RetrieveRequestTag<'a>>,
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<RetrieveRequestContentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<RetrieveRequestSort>,
    #[serde(rename = "detailType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<RetrieveRequestDetailType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
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

#[derive(Debug, Serialize_repr)]
#[repr(u8)]
pub enum RetrieveRequestFavorite {
    UnFavorited = 0,
    Favorited = 1,
}

#[derive(Debug)]
pub enum RetrieveRequestTag<'a> {
    Tagged(&'a str),
    Untagged(&'a str),
}

impl<'a> Serialize for RetrieveRequestTag<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            RetrieveRequestTag::Tagged(s) => serializer.serialize_str(s),
            RetrieveRequestTag::Untagged(s) => serializer.serialize_str(&format!("_{}_", s)),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum RetrieveRequestContentType {
    #[serde(rename = "article")]
    Article,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "image")]
    Image,
}

#[derive(Debug, Serialize)]
pub enum RetrieveRequestSort {
    #[serde(rename = "newest")]
    Newest,
    #[serde(rename = "oldest")]
    Oldest,
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "site")]
    Site,
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
            state: None,
            favorite: None,
            tag: None,
            content_type: None,
            sort: None,
            detail_type: Some(RetrieveRequestDetailType::Simple),
            search: None,
            domain: None,
            since: None,
            count: Some(123),
            offset: None,
        };
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "detailType": "simple",
  "count": 123
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

    #[test]
    fn retrieve_request_favorite() -> anyhow::Result<()> {
        let mut request = build_retrieve_request("consumer_key1", "access_token1");

        request.favorite = Some(RetrieveRequestFavorite::UnFavorited);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "favorite": 0
}"#
        );

        request.favorite = Some(RetrieveRequestFavorite::Favorited);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "favorite": 1
}"#
        );

        Ok(())
    }

    #[test]
    fn retrieve_request_tag() -> anyhow::Result<()> {
        let mut request = build_retrieve_request("consumer_key1", "access_token1");

        request.tag = Some(RetrieveRequestTag::Tagged("tag1"));
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "tag": "tag1"
}"#
        );

        request.tag = Some(RetrieveRequestTag::Untagged("tag1"));
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "tag": "_tag1_"
}"#
        );

        Ok(())
    }

    #[test]
    fn retrieve_request_content_type() -> anyhow::Result<()> {
        let mut request = build_retrieve_request("consumer_key1", "access_token1");

        request.content_type = Some(RetrieveRequestContentType::Article);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "contentType": "article"
}"#
        );

        request.content_type = Some(RetrieveRequestContentType::Video);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "contentType": "video"
}"#
        );

        request.content_type = Some(RetrieveRequestContentType::Image);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "contentType": "image"
}"#
        );

        Ok(())
    }

    #[test]
    fn retrieve_request_sort() -> anyhow::Result<()> {
        let mut request = build_retrieve_request("consumer_key1", "access_token1");

        request.sort = Some(RetrieveRequestSort::Newest);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "sort": "newest"
}"#
        );

        request.sort = Some(RetrieveRequestSort::Oldest);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "sort": "oldest"
}"#
        );

        request.sort = Some(RetrieveRequestSort::Title);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "sort": "title"
}"#
        );

        request.sort = Some(RetrieveRequestSort::Site);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "sort": "site"
}"#
        );

        Ok(())
    }

    #[test]
    fn retrieve_request_search() -> anyhow::Result<()> {
        let mut request = build_retrieve_request("consumer_key1", "access_token1");

        request.search = Some("s");
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "search": "s"
}"#
        );

        Ok(())
    }

    #[test]
    fn retrieve_request_domain() -> anyhow::Result<()> {
        let mut request = build_retrieve_request("consumer_key1", "access_token1");

        request.domain = Some("example.com");
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "domain": "example.com"
}"#
        );

        Ok(())
    }

    #[test]
    fn retrieve_request_since() -> anyhow::Result<()> {
        let mut request = build_retrieve_request("consumer_key1", "access_token1");

        request.since = Some(1_234_567_890_u64);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "since": 1234567890
}"#
        );

        Ok(())
    }

    #[test]
    fn retrieve_request_offset() -> anyhow::Result<()> {
        let mut request = build_retrieve_request("consumer_key1", "access_token1");

        request.offset = Some(10);
        assert_eq!(
            serde_json::to_string_pretty(&request)?,
            r#"{
  "consumer_key": "consumer_key1",
  "access_token": "access_token1",
  "offset": 10
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
            favorite: None,
            tag: None,
            content_type: None,
            sort: None,
            detail_type: None,
            search: None,
            domain: None,
            since: None,
            count: None,
            offset: None,
        }
    }
}
