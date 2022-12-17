use std::{collections::HashMap, env};

use anyhow::bail;
use reqwest::{Client, Method};
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};
use tracing::debug;

use crate::{
    authorize::AccessTokenResponse,
    domain::ScheduledTweet,
    store::{Token, TweetQueueStore},
};

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

    if response.status() != 201 {
        let text = response.text().await?;
        bail!(text);
    }

    #[allow(dead_code)]
    #[derive(Debug, serde::Deserialize)]
    struct ResponseData {
        id: String,
        text: String,
    }

    #[allow(dead_code)]
    #[derive(Debug, serde::Deserialize)]
    struct Response {
        data: ResponseData,
    }

    let json: Response = response.json().await?;

    println!("{:?}", json);

    Ok(())
}

async fn ensure_token(store: &TweetQueueStore) -> anyhow::Result<Token> {
    let token = store.read_token().await?;
    match token {
        Some(token) => {
            let expires = OffsetDateTime::parse(&token.expires, &Rfc3339)?;
            if OffsetDateTime::now_utc() < expires - Duration::seconds(10) {
                Ok(token)
            } else {
                // use refresh token
                let client_id = env::var("TWITTER_CLIENT_ID")?;
                let client_secret = env::var("TWITTER_CLIENT_SECRET")?;
                let response = Client::builder()
                    .build()?
                    .request(Method::POST, "https://api.twitter.com/2/oauth2/token")
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .basic_auth(&client_id, Some(&client_secret))
                    .form(&{
                        let mut form = HashMap::new();
                        form.insert("grant_type", "refresh_token");
                        form.insert("refresh_token", token.refresh_token.as_str());
                        form
                    })
                    .send()
                    .await?;
                let access_token_response: AccessTokenResponse = response.json().await?;
                debug!("{:?}", access_token_response);

                let token =
                    access_token_response.try_into(OffsetDateTime::now_utc().unix_timestamp())?;

                store.write_token(&token).await?;

                Ok(token)
            }
        }
        None => bail!("Use `twiq-light queue authorize`"),
    }
}

pub async fn run(store: TweetQueueStore) -> anyhow::Result<()> {
    let token = ensure_token(&store).await?;
    debug!("{:?}", token);
    let mut queue = store.read_all().await?;
    if let Some(item) = queue.pop_front() {
        post_tweet(&token.access_token, item).await?;
        store.write_all(&queue).await?;
    }
    Ok(())
}
