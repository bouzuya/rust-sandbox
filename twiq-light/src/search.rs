use tracing::instrument;

use crate::store::TweetStore;

#[instrument(skip_all)]
pub async fn run(store: TweetStore, query: Option<String>) -> anyhow::Result<()> {
    let data = store.read_all().await?;
    println!("{}", data.len());

    let mut result = vec![];
    for (_, tweet) in data {
        match query {
            Some(ref query) => {
                if query.split_whitespace().all(|q| tweet.text.contains(q)) {
                    result.push(tweet);
                }
            }
            None => {
                result.push(tweet);
            }
        }
    }
    result.sort_by_key(|t| t.at.clone());

    for tweet in result {
        println!("{}", serde_json::to_string(&tweet)?);
    }

    Ok(())
}
