use anyhow::bail;
use tracing::instrument;

use crate::{data::ScheduledTweet, store::TweetQueueStore};

#[instrument(skip_all)]
pub async fn run(
    store: TweetQueueStore,
    tweet: String,
    reply: Option<String>,
) -> anyhow::Result<()> {
    if tweet.chars().count() > 140 {
        bail!("The length of the tweet exceeded 140 characters");
    }
    let mut queue = store.read_all().await?;
    queue.push_back(ScheduledTweet { text: tweet, reply });
    store.write_all(&queue).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    fn check(s: &str) -> bool {
        if s.len() > 6 * 140 {
            return false;
        }
        s.chars()
            .map(|c| if c.is_ascii_alphanumeric() { 1 } else { 2 })
            .sum::<usize>()
            <= 280
    }

    #[test]
    fn test() {
        assert!(check("a".repeat(280).as_str()));
        assert!(check("0".repeat(280).as_str()));
    }
}
