use std::env;

use reqwest::{Client, Method};
use tracing::debug;

use crate::{domain::ScheduledTweet, store::TweetQueueStore};

async fn post_tweet(bearer_token: &str, tweet: ScheduledTweet) -> anyhow::Result<()> {
    let url = "https://api.twitter.com/2/tweets";
    let response = Client::builder()
        .build()?
        .request(Method::POST, url)
        .bearer_auth(bearer_token)
        .json(&tweet)
        .send()
        .await?;

    debug!("response.status={:?}", response.status());
    Ok(response.json().await?)
}

pub async fn run(store: TweetQueueStore) -> anyhow::Result<()> {
    let bearer_token = env::var("TWITTER_BEARER_TOKEN2")?;
    debug!(bearer_token);
    let mut queue = store.read_all().await?;
    if let Some(item) = queue.pop_front() {
        post_tweet(&bearer_token, item).await?;
        store.write_all(&queue).await?;
    }
    Ok(())
}
