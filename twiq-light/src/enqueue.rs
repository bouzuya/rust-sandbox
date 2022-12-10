use crate::{domain::ScheduledTweet, store::TweetQueueStore};

pub async fn run(store: TweetQueueStore, tweet: String) -> anyhow::Result<()> {
    let mut queue = store.read_all()?;
    queue.push_back(ScheduledTweet { text: tweet });
    store.write_all(&queue)?;
    Ok(())
}
