use tracing::instrument;

use crate::store::TweetQueueStore;

#[instrument(skip_all)]
pub async fn run(store: TweetQueueStore, index: usize) -> anyhow::Result<()> {
    let mut queue = store.read_all().await?;
    queue.remove(index);
    store.write_all(&queue).await?;
    Ok(())
}
