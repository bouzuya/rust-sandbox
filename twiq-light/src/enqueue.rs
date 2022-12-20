use tracing::instrument;

use crate::{domain::ScheduledTweet, store::TweetQueueStore};

#[instrument(skip_all)]
pub async fn run(store: TweetQueueStore, tweet: String) -> anyhow::Result<()> {
    let mut queue = store.read_all().await?;
    queue.push_back(ScheduledTweet { text: tweet });
    store.write_all(&queue).await?;
    Ok(())
}
