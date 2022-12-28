use anyhow::bail;
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};
use tracing::{debug, instrument};

use crate::{
    domain::MyTweet,
    store::TweetQueueStore,
    token::Token,
    tweet_store::TweetStore,
    twitter::{
        self, GetUsersIdTweetsPathParams, GetUsersIdTweetsQueryParams, TweetResponseDataItem,
    },
};

// TODO: duplicate
async fn ensure_token(
    store: &TweetQueueStore,
    client_id: &str,
    client_secret: &str,
) -> anyhow::Result<Token> {
    let token = store.read_token().await?;
    match token {
        Some(token) => {
            let expires = OffsetDateTime::parse(&token.expires, &Rfc3339)?;
            if OffsetDateTime::now_utc() < expires - Duration::seconds(10) {
                Ok(token)
            } else {
                // use refresh token
                let access_token_response = twitter::refresh_access_token(
                    client_id,
                    client_secret,
                    token.refresh_token.as_str(),
                )
                .await?;
                debug!("{:?}", access_token_response);

                let token = Token::try_from(
                    access_token_response,
                    OffsetDateTime::now_utc().unix_timestamp(),
                )?;

                store.write_token(&token).await?;

                Ok(token)
            }
        }
        None => bail!("Use `twiq-light queue authorize`"),
    }
}

#[instrument(skip_all)]
pub async fn run(
    store: TweetStore,
    tweet_queue_store: TweetQueueStore,
    client_id: String,
    client_secret: String,
) -> anyhow::Result<()> {
    let token = ensure_token(&tweet_queue_store, &client_id, &client_secret).await?;
    debug!("{:?}", token);
    let mut data = store.read_all().await?;
    let last_id_str = {
        let mut at_id = data
            .values()
            .map(|t| (t.at.as_ref(), t.id_str.as_ref()))
            .collect::<Vec<(&str, &str)>>();
        at_id.sort();
        at_id.last().cloned().map(|(_, id_str)| id_str.to_owned())
    };
    debug!(last_id_str);

    let path_params = GetUsersIdTweetsPathParams {
        id: "125962981".to_owned(),
    };
    let mut tweets = vec![];
    let mut response = twitter::get_users_id_tweets(
        &token.access_token,
        &path_params,
        &GetUsersIdTweetsQueryParams {
            max_results: Some(100),
            pagination_token: None,
        },
    )
    .await?;
    while let Some(ref pagination_token) = response.meta.next_token {
        if let Some(ref id_str) = last_id_str {
            if response.data.iter().any(|d| &d.id == id_str) {
                break;
            }
        }
        tweets.extend(response.data);
        response = twitter::get_users_id_tweets(
            &token.access_token,
            &path_params,
            &GetUsersIdTweetsQueryParams {
                max_results: Some(100),
                pagination_token: Some(pagination_token.to_owned()),
            },
        )
        .await?;
    }
    tweets.extend(if let Some(ref id_str) = last_id_str {
        response
            .data
            .into_iter()
            .take_while(|d| &d.id != id_str)
            .collect::<Vec<TweetResponseDataItem>>()
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

    store.write_all(&data).await?;

    Ok(())
}
