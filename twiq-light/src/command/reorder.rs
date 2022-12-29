use tracing::instrument;

use crate::store::TweetQueueStore;

#[instrument(skip_all)]
pub async fn run(store: TweetQueueStore, src: usize, dst: usize) -> anyhow::Result<()> {
    if src == dst {
        return Ok(());
    }

    let mut queue = store.read_all().await?;
    if let Some(x) = queue.remove(src) {
        queue.insert(dst, x);
    }
    store.write_all(&queue).await?;
    Ok(())
}
