use tracing::{debug, instrument};

use crate::{
    store::CredentialStore,
    twitter::{self, PostTweetsRequestBody, PostTweetsRequestBodyReply},
};

#[instrument(skip_all)]
pub async fn run(
    credential_store: CredentialStore,
    text: String,
    reply: Option<String>,
) -> anyhow::Result<()> {
    let token = credential_store.ensure_token().await?;
    debug!("{:?}", token);
    twitter::post_tweets(
        &token.access_token,
        PostTweetsRequestBody {
            text: Some(text),
            reply: reply.map(|r| PostTweetsRequestBodyReply {
                in_reply_to_tweet_id: Some(r),
            }),
        },
    )
    .await?;

    Ok(())
}
