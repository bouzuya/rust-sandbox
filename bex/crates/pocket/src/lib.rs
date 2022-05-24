mod authorization;
mod retrieve_request;

use std::collections::HashMap;

pub use authorization::*;
use hyper::StatusCode;
use reqwest::Response;
pub use retrieve_request::*;
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

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveResponse {
    pub complete: Option<u16>,
    pub error: Option<Value>,
    pub list: HashMap<String, RetrieveItemResponse>,
    pub search_meta: Option<Value>,
    pub since: Option<u64>,
    pub status: u16,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveItemResponse {
    pub authors: Option<HashMap<String, RetrieveItemAuthorResponse>>,
    pub domain_metadata: Option<RetrieveItemDomainMetadataResponse>,
    pub excerpt: Option<String>,
    pub favorite: String,
    pub given_title: String,
    pub given_url: String,
    pub has_image: Option<String>,
    pub has_video: Option<String>,
    pub image: Option<RetrieveItemImageResponse>,
    pub images: Option<HashMap<String, RetrieveItemImagesItemResponse>>,
    pub is_article: Option<String>,
    pub is_index: Option<String>,
    pub item_id: String,
    pub lang: Option<String>,
    pub listen_duration_estimate: Option<u16>,
    pub resolved_id: String,
    pub resolved_title: Option<String>,
    pub resolved_url: Option<String>,
    pub sort_id: Option<u64>,
    pub status: String,
    pub tags: Option<HashMap<String, RetrieveItemTagsItemResponse>>,
    pub time_added: Option<String>,
    pub time_favorited: Option<String>,
    pub time_read: Option<String>,
    pub time_to_read: Option<u64>,
    pub time_updated: Option<String>,
    pub top_image_url: Option<String>,
    pub videos: Option<HashMap<String, RetrieveItemVideosItemResponse>>,
    pub word_count: Option<String>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveItemAuthorResponse {
    author_id: String,
    item_id: String,
    name: String,
    url: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveItemDomainMetadataResponse {
    greyscale_logo: String,
    logo: String,
    name: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveItemImageResponse {
    height: String,
    item_id: String,
    src: String,
    width: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveItemImagesItemResponse {
    caption: String,
    credit: String,
    height: String,
    image_id: String,
    item_id: String,
    src: String,
    width: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveItemTagsItemResponse {
    item_id: String,
    tag: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveItemVideosItemResponse {
    height: String,
    item_id: String,
    // It seems to be required but is not included in the examples in the document.
    length: Option<String>,
    src: String,
    r#type: String,
    vid: String,
    video_id: String,
    width: String,
}

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
