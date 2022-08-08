#[cfg(test)]
mod tests {
    use reqwest::{Client, Method};

    #[tokio::test]
    async fn it_works() -> anyhow::Result<()> {
        #[derive(Debug, serde::Deserialize)]
        struct Json {
            date: String,
        }
        let response = reqwest::get("https://blog.bouzuya.net/2022/08/07/index.json").await?;
        let json: Json = response.json().await?;
        assert_eq!(json.date, "2022-08-07");
        Ok(())
    }

    #[tokio::test]
    async fn get_user() -> anyhow::Result<()> {
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

        let bearer_token = std::env::var("TWITTER_BEARER_TOKEN")?;
        let url = format!("https://api.twitter.com/2/users/by/username/{}", "bouzuya");
        let response = Client::builder()
            .build()?
            .request(Method::GET, url)
            .bearer_auth(bearer_token)
            .send()
            .await?;
        let json: UserResponse = response.json().await?;
        assert_eq!(json.data.id, "125962981");
        assert_eq!(json.data.name, "bouzuya");
        assert_eq!(json.data.username, "bouzuya");
        assert_ne!(serde_json::to_string(&json)?, "");
        Ok(())
    }
}
