use std::time::Instant;

struct MyTweet {
    id_str: String,
    retweet: bool,
    at: Instant,
    user_name: String,
    user_icon: String,
    text: String,
    urls: Vec<MyTweetUrl>,
}

struct MyTweetUrl {
    display_url: String,
    expand_url: String,
    indices: Vec<usize>,
    url: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TweetResponse {
    data: Vec<TweetResponseData>,
    meta: TweetResponseMeta,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TweetResponseData {
    id: String,
    text: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TweetResponseMeta {
    newest_id: String,
    next_token: Option<String>,
    oldest_id: String,
    previous_token: Option<String>,
    result_count: usize,
}

#[cfg(test)]
mod tests {
    use reqwest::{Client, Method};

    use crate::TweetResponse;

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

    #[tokio::test]
    async fn get_tweets() -> anyhow::Result<()> {
        let bearer_token = std::env::var("TWITTER_BEARER_TOKEN")?;
        let id = "125962981";
        let url = format!("https://api.twitter.com/2/users/{}/tweets", id);
        let response = Client::builder()
            .build()?
            .request(Method::GET, url)
            .bearer_auth(bearer_token)
            .send()
            .await?;
        let json: TweetResponse = response.json().await?;
        assert!(json.data.iter().any(|i| i.id == "1556520585856880640"));
        assert_eq!(
            json.meta.next_token,
            Some("7140dibdnow9c7btw422nobb6nigqr50544iaynyqphkg".to_string())
        );
        Ok(())
    }

    #[tokio::test]
    async fn get_tweets_max_results_and_pagination_token() -> anyhow::Result<()> {
        let bearer_token = std::env::var("TWITTER_BEARER_TOKEN")?;
        let id = "125962981";
        let url = format!(
            "https://api.twitter.com/2/users/{}/tweets?max_results={}&pagination_token={}",
            id, 100, "7140dibdnow9c7btw422nobb6nigqr50544iaynyqphkg"
        );
        let response = Client::builder()
            .build()?
            .request(Method::GET, url)
            .bearer_auth(bearer_token)
            .send()
            .await?;
        let json: TweetResponse = response.json().await?;
        assert_eq!(json.data.len(), 100);
        assert!(!json.data.iter().any(|i| i.id == "1556520585856880640"));
        Ok(())
    }
}
