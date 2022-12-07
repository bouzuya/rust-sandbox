use std::env;

use reqwest::{Client, Method};
use tracing::{debug, instrument};

use crate::{domain::MyTweet, store::TweetStore};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TweetResponse {
    data: Vec<TweetResponseData>,
    meta: TweetResponseMeta,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TweetResponseData {
    created_at: String, // "2021-03-23T16:59:18.000Z"
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

async fn get_tweets(
    bearer_token: &str,
    pagination_token: Option<&str>,
) -> anyhow::Result<TweetResponse> {
    let id = "125962981";
    let url = format!(
        "https://api.twitter.com/2/users/{}/tweets?max_results={}&tweet.fields=created_at{}",
        id,
        100,
        match pagination_token {
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

    debug!("response.status={:?}", response.status());
    Ok(response.json().await?)
}

#[instrument(skip_all)]
pub async fn run(store: TweetStore) -> anyhow::Result<()> {
    let mut data = store.read_all()?;
    let last_id_str = {
        let mut at_id = data
            .iter()
            .map(|(_, t)| (t.at.as_ref(), t.id_str.as_ref()))
            .collect::<Vec<(&str, &str)>>();
        at_id.sort();
        at_id.last().cloned().map(|(_, id_str)| id_str.to_owned())
    };
    debug!(last_id_str);

    let bearer_token = env::var("TWITTER_BEARER_TOKEN")?;
    debug!(bearer_token);
    let mut tweets = vec![];
    let mut response = get_tweets(&bearer_token, None).await?;
    while let Some(ref pagination_token) = response.meta.next_token {
        if let Some(ref id_str) = last_id_str {
            if response.data.iter().any(|d| &d.id == id_str) {
                break;
            }
        }
        tweets.extend(response.data);
        response = get_tweets(&bearer_token, Some(pagination_token.as_ref())).await?;
    }
    tweets.extend(if let Some(ref id_str) = last_id_str {
        response
            .data
            .into_iter()
            .take_while(|d| &d.id != id_str)
            .collect::<Vec<TweetResponseData>>()
    } else {
        response.data
    });

    debug!("tweets.len={}", tweets.len());
    for tweet in tweets.into_iter().map(|t| MyTweet {
        id_str: t.id,
        at: t.created_at,
        text: t.text,
    }) {
        debug!("{:?}", tweet);
        data.insert(tweet.id_str.clone(), tweet);
    }

    store.write_all(&data)?;

    Ok(())
}
