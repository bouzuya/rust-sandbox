use std::{collections::BTreeMap, env, path::PathBuf};

use crate::{
    data::MyTweet,
    storage::{fs::FsStorage, Storage},
};

#[derive(Debug)]
pub struct TweetStore {
    storage: FsStorage,
}

impl Default for TweetStore {
    fn default() -> Self {
        let storage = FsStorage::new(PathBuf::from(&env::var("HOME").expect("env HOME")));
        Self { storage }
    }
}

impl TweetStore {
    pub async fn read_all(&self) -> anyhow::Result<BTreeMap<String, MyTweet>> {
        let item = self
            .storage
            .get_item(PathBuf::from("twiq-light.json"))
            .await?;
        Ok(match item {
            None => BTreeMap::default(),
            Some(s) => serde_json::from_str(&s)?,
        })
    }

    pub async fn write_all(&self, data: &BTreeMap<String, MyTweet>) -> anyhow::Result<()> {
        self.storage
            .set_item(
                PathBuf::from("twiq-light.json"),
                serde_json::to_string(data)?,
            )
            .await
    }
}
