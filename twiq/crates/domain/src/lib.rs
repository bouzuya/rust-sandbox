pub mod aggregate;
mod value;

#[derive(Debug)]
pub struct MyTweet {
    pub id_str: String,
    pub retweet: bool,
    pub at: String,
    pub author_id: String,
    pub text: String,
    pub entities: MyTweetEntities,
}

#[derive(Debug)]
pub struct MyTweetEntities {
    pub hashtags: Vec<MyTweetHashtag>,
    // pub media: Vec<MyTweetMedia>,
    pub mentions: Vec<MyTweetMention>,
    pub urls: Vec<MyTweetUrl>,
}

#[derive(Debug)]
pub struct MyTweetHashtag {
    pub end: usize,
    pub start: usize,
    pub tag: String,
}

// #[derive(Debug)]
// pub struct MyTweetMedia {
//     pub end: usize,
//     pub start: usize,
//     pub id_str: String,
//     // pic.twitter.com/psP0pA1QUy
//     pub display_url: String,
//     // https://twitter.com/bouzuya/status/576785452637691904/photo/1
//     pub expanded_url: String,
//     // (media_url_https)
//     // https://pbs.twimg.com/media/CAEnSmgUwAAG1TY.png
//     pub media_url: String,
//     pub sizes: // ...
// }

#[derive(Debug)]
pub struct MyTweetMention {
    pub end: usize,
    pub start: usize,
    pub id_str: String,
}

#[derive(Debug)]
pub struct MyTweetUrl {
    pub display_url: String,
    pub end: usize,
    pub expanded_url: String,
    pub start: usize,
    pub url: String,
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
    // use reqwest::{Client, Method};

    // use crate::TweetResponse;

    // #[tokio::test]
    // async fn get_tweets() -> anyhow::Result<()> {
    //     let bearer_token = std::env::var("TWITTER_BEARER_TOKEN")?;
    //     let id = "125962981";
    //     let url = format!("https://api.twitter.com/2/users/{}/tweets", id);
    //     let response = Client::builder()
    //         .build()?
    //         .request(Method::GET, url)
    //         .bearer_auth(bearer_token)
    //         .send()
    //         .await?;
    //     let json: TweetResponse = response.json().await?;
    //     assert!(json.data.iter().any(|i| i.id == "1556520585856880640"));
    //     assert_eq!(
    //         json.meta.next_token,
    //         Some("7140dibdnow9c7btw422nobb6nigqr50544iaynyqphkg".to_string())
    //     );
    //     Ok(())
    // }

    // #[tokio::test]
    // async fn get_tweets_max_results_and_pagination_token() -> anyhow::Result<()> {
    //     let bearer_token = std::env::var("TWITTER_BEARER_TOKEN")?;
    //     let id = "125962981";
    //     let url = format!(
    //         "https://api.twitter.com/2/users/{}/tweets?max_results={}&pagination_token={}",
    //         id, 100, "7140dibdnow9c7btw422nobb6nigqr50544iaynyqphkg"
    //     );
    //     let response = Client::builder()
    //         .build()?
    //         .request(Method::GET, url)
    //         .bearer_auth(bearer_token)
    //         .send()
    //         .await?;
    //     let json: TweetResponse = response.json().await?;
    //     assert_eq!(json.data.len(), 100);
    //     assert!(!json.data.iter().any(|i| i.id == "1556520585856880640"));
    //     Ok(())
    // }
}
