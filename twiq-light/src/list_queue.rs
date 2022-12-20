use tracing::instrument;

use crate::store::TweetQueueStore;

#[instrument(skip_all)]
pub async fn run(store: TweetQueueStore) -> anyhow::Result<()> {
    let queue = store.read_all().await?;
    for scheduled_tweet in queue {
        println!("{}", serde_json::to_string(&scheduled_tweet)?);
    }
    Ok(())
}
