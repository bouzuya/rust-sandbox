use std::collections::VecDeque;

use crate::domain::ScheduledTweet;

async fn load_queue() -> anyhow::Result<VecDeque<ScheduledTweet>> {
    Ok(VecDeque::new())
}

async fn post_tweet(tweet: ScheduledTweet) -> anyhow::Result<()> {
    Ok(())
}

async fn save_queue(_queue: VecDeque<ScheduledTweet>) -> anyhow::Result<()> {
    Ok(())
}

pub async fn run() -> anyhow::Result<()> {
    let mut queue = load_queue().await?;
    if let Some(item) = queue.pop_front() {
        post_tweet(item).await?;
        save_queue(queue).await?;
    }
    Ok(())
}
