use tracing::{debug, instrument};

use crate::{
    data::MyTweet,
    store::{CredentialStore, TweetStore},
    twitter::{
        self, GetUsersIdTweetsPathParams, GetUsersIdTweetsQueryParams, TweetResponseDataItem,
    },
};

#[instrument(skip_all)]
pub async fn run(store: TweetStore, credential_store: CredentialStore) -> anyhow::Result<()> {
    let token = credential_store.ensure_token().await?;
    debug!("{:?}", token);
    let user = twitter::get_users_me(&token.access_token).await?;
    debug!("{:?}", user);
    let user_id = user.data.id;
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

    let path_params = GetUsersIdTweetsPathParams { id: user_id };
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
