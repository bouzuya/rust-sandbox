use std::env;

use tracing::{debug, instrument};

use crate::{
    domain::MyTweet,
    store::TweetStore,
    twitter::{
        self, GetUsersIdTweetsPathParams, GetUsersIdTweetsQueryParams, TweetResponseDataItem,
    },
};

#[instrument(skip_all)]
pub async fn run(store: TweetStore) -> anyhow::Result<()> {
    let mut data = store.read_all()?;
    let last_id_str = {
        let mut at_id = data
            .values()
            .map(|t| (t.at.as_ref(), t.id_str.as_ref()))
            .collect::<Vec<(&str, &str)>>();
        at_id.sort();
        at_id.last().cloned().map(|(_, id_str)| id_str.to_owned())
    };
    debug!(last_id_str);

    let bearer_token = env::var("TWITTER_BEARER_TOKEN")?;
    debug!(bearer_token);
    let path_params = GetUsersIdTweetsPathParams {
        id: "125962981".to_owned(),
    };
    let mut tweets = vec![];
    let mut response = twitter::get_users_id_tweets(
        &bearer_token,
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
            &bearer_token,
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

    store.write_all(&data)?;

    Ok(())
}
