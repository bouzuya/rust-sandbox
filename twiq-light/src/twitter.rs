use std::collections::HashMap;

use anyhow::bail;
use reqwest::{Client, Method};

// <https://www.rfc-editor.org/rfc/rfc6749#section-5.1>
#[derive(Debug, serde::Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: String, // ...
    #[allow(dead_code)]
    pub token_type: String, // "bearer"
    pub expires_in: Option<u32>, // 7200
    #[allow(dead_code)]
    pub scope: Option<String>, // "tweet.write users.read tweet.read offline.access"
    pub refresh_token: Option<String>, // ...
}

pub struct GetUsersIdTweetsPathParams {
    pub id: String,
}

pub struct GetUsersIdTweetsQueryParams {
    pub max_results: Option<u8>, // 5..=100 (default: 10)
    pub pagination_token: Option<String>,
    // ...
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TweetResponse {
    pub data: Vec<TweetResponseDataItem>,
    pub meta: TweetResponseMeta,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TweetResponseDataItem {
    pub created_at: String, // "2021-03-23T16:59:18.000Z"
    pub id: String,
    pub text: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TweetResponseMeta {
    pub newest_id: String,
    pub next_token: Option<String>,
    pub oldest_id: String,
    pub previous_token: Option<String>,
    pub result_count: usize,
}

// <https://developer.twitter.com/en/docs/twitter-api/tweets/timelines/api-reference/get-users-id-tweets>
pub async fn get_users_id_tweets(
    bearer_token: &str,
    path_params: &GetUsersIdTweetsPathParams,
    query_params: &GetUsersIdTweetsQueryParams,
) -> anyhow::Result<TweetResponse> {
    let url = format!(
        "https://api.twitter.com/2/users/{}/tweets?tweet.fields=created_at{}{}",
        path_params.id,
        match query_params.max_results {
            Some(t) => format!("&max_results={}", t),
            None => "".to_owned(),
        },
        match query_params.pagination_token.as_ref() {
            Some(t) => format!("&pagination_token={}", t),
            None => "".to_owned(),
        }
    );
    let response = Client::builder()
        .build()?
        .request(Method::GET, url)
        .bearer_auth(bearer_token)
        .send()
        .await?;
    if response.status().is_success() {
        Ok(response.json().await?)
    } else {
        bail!("response.status={:?}", response.status());
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UserResponse {
    pub data: UserResponseData,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UserResponseData {
    pub id: String,
    pub name: String,
    pub username: String,
}

// <https://developer.twitter.com/en/docs/twitter-api/users/lookup/api-reference/get-users-me>
pub async fn get_users_me(bearer_token: &str) -> anyhow::Result<UserResponse> {
    let url = "https://api.twitter.com/2/users/me";
    let response = Client::builder()
        .build()?
        .request(Method::GET, url)
        .bearer_auth(bearer_token)
        .send()
        .await?;
    if response.status().is_success() {
        Ok(response.json().await?)
    } else {
        bail!("response.status={:?}", response.status());
    }
}

// <https://www.rfc-editor.org/rfc/rfc6749#section-5>
pub async fn issue_token(
    client_id: &str,
    client_secret: &str,
    code: &str,
    redirect_uri: &str,
    code_verifier: &str,
) -> anyhow::Result<AccessTokenResponse> {
    let response = Client::builder()
        .build()?
        .request(Method::POST, "https://api.twitter.com/2/oauth2/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .basic_auth(client_id, Some(client_secret))
        .form(&{
            let mut form = HashMap::new();
            form.insert("code", code);
            form.insert("grant_type", "authorization_code");
            form.insert("redirect_uri", redirect_uri);
            form.insert("code_verifier", code_verifier);
            form
        })
        .send()
        .await?;
    if response.status().is_success() {
        Ok(response.json().await?)
    } else {
        bail!("response.status={:?}", response.status());
    }
}

#[derive(Debug, serde::Serialize)]
pub struct PostTweetsRequestBody {
    pub text: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct PostTweetsResponseBody {
    pub data: PostTweetsResponseBodyData,
}

#[derive(Debug, serde::Deserialize)]
pub struct PostTweetsResponseBodyData {
    pub id: String,
    pub text: String,
}

// <https://developer.twitter.com/en/docs/twitter-api/tweets/manage-tweets/api-reference/post-tweets>
pub async fn post_tweets(
    bearer_token: &str,
    tweet: PostTweetsRequestBody,
) -> anyhow::Result<PostTweetsResponseBody> {
    let url = "https://api.twitter.com/2/tweets";
    let response = Client::builder()
        .build()?
        .request(Method::POST, url)
        .bearer_auth(bearer_token)
        .json(&tweet)
        .send()
        .await?;
    if response.status().is_success() {
        Ok(response.json().await?)
    } else {
        let text = response.text().await?;
        bail!(text);
    }
}

// <https://www.rfc-editor.org/rfc/rfc6749#section-6>
pub async fn refresh_access_token(
    client_id: &str,
    client_secret: &str,
    refresh_token: &str,
) -> anyhow::Result<AccessTokenResponse> {
    let response = Client::builder()
        .build()?
        .request(Method::POST, "https://api.twitter.com/2/oauth2/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .basic_auth(client_id, Some(client_secret))
        .form(&{
            let mut form = HashMap::new();
            form.insert("grant_type", "refresh_token");
            form.insert("refresh_token", refresh_token);
            // scope is optional
            form
        })
        .send()
        .await?;
    if response.status().is_success() {
        Ok(response.json().await?)
    } else {
        bail!("response.status={}", response.status());
    }
}
