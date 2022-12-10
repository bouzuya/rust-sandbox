use crate::{domain::ScheduledTweet, store::TweetQueueStore};

async fn post_tweet(tweet: ScheduledTweet) -> anyhow::Result<()> {
    println!("TODO: post {:?}", tweet);
    Ok(())
}

pub async fn run(store: TweetQueueStore) -> anyhow::Result<()> {
    let mut queue = store.read_all()?;
    if let Some(item) = queue.pop_front() {
        post_tweet(item).await?;
        store.write_all(&queue)?;
    }
    Ok(())
}
