use std::{collections::BTreeMap, env, fs, path::Path};

use crate::domain::MyTweet;

pub async fn run(query: Option<String>) -> anyhow::Result<()> {
    let path = Path::new(&env::var("HOME")?).join("twiq-light.json");
    let data = {
        if !path.exists() {
            BTreeMap::new()
        } else {
            let s = fs::read_to_string(&path)?;
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
