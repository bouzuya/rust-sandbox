use tracing::{debug, instrument};

use crate::{
    store::{CredentialStore, TweetQueueStore},
    twitter::{self, PostTweetsRequestBody, PostTweetsRequestBodyReply},
};

#[instrument(skip_all)]
pub async fn run(store: TweetQueueStore, credential_store: CredentialStore) -> anyhow::Result<()> {
    let token = credential_store.ensure_token().await?;
    debug!("{:?}", token);
    let mut queue = store.read_all().await?;
    if let Some(item) = queue.pop_front() {
        twitter::post_tweets(
            &token.access_token,
            PostTweetsRequestBody {
                text: Some(item.text),
                reply: item.reply.map(|r| PostTweetsRequestBodyReply {
                    in_reply_to_tweet_id: Some(r),
                }),
            },
        )
        .await?;
        store.write_all(&queue).await?;
    }
    Ok(())
}
