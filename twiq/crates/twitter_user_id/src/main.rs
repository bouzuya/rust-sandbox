use std::env;

use reqwest::{Client, Method, Url};

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

async fn get_user(bearer_token: &str, username: &str) -> anyhow::Result<UserResponse> {
    let mut url = Url::parse("https://api.twitter.com")?;
    url.set_path(&format!("/2/users/by/username/{}", username));
    let response = Client::builder()
        .build()?
        .request(Method::GET, url)
        .bearer_auth(bearer_token)
        .send()
        .await?;
    Ok(response.json().await?)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let bearer_token = env::var("TWITTER_BEARER_TOKEN")?;
    let username = &args[1];
    let json = get_user(&bearer_token, username).await?;
    println!("{}", serde_json::to_string(&json)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "TWITTER_BEARER_TOKEN env"]
    async fn test() -> anyhow::Result<()> {
        let bearer_token = env::var("TWITTER_BEARER_TOKEN")?;
        let username = "bouzuya";
        let json = get_user(bearer_token.as_str(), username).await?;
        assert_eq!(json.data.id, "125962981");
        assert_eq!(json.data.name, "bouzuya");
        assert_eq!(json.data.username, "bouzuya");
        Ok(())
    }
}
