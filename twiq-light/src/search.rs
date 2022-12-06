use std::{collections::BTreeMap, fs};

use crate::{domain::MyTweet, store::TweetStore};

pub async fn run(store: TweetStore, query: Option<String>) -> anyhow::Result<()> {
    let data = {
        if !store.path().exists() {
            BTreeMap::new()
        } else {
            let s = fs::read_to_string(store.path())?;
            let data: BTreeMap<String, MyTweet> = serde_json::from_str(&s)?;
            data
        }
    };

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
