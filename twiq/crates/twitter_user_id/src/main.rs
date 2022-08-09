use std::env;

use reqwest::{Client, Method};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct UserResponse {
    data: UserResponseData,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct UserResponseData {
    id: String,
    name: String,
    username: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let bearer_token = std::env::var("TWITTER_BEARER_TOKEN")?;
    let url = format!("https://api.twitter.com/2/users/by/username/{}", args[1]);
    let response = Client::builder()
        .build()?
        .request(Method::GET, url)
        .bearer_auth(bearer_token)
        .send()
        .await?;
    let json: UserResponse = response.json().await?;
    println!("{}", serde_json::to_string(&json)?);
    Ok(())
}
