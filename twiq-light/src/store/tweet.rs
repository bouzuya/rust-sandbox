use std::{collections::BTreeMap, env, path::PathBuf};

use xdg::BaseDirectories;

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
        let state_dir = match env::var_os("TWIQ_LIGHT_STATE_DIR") {
            None => BaseDirectories::with_prefix(Self::PREFIX)
                .expect("xdg")
                .get_state_home(),
            Some(state_dir) => PathBuf::from(state_dir),
        };
        let storage = FsStorage::new(state_dir);
        Self { storage }
    }
}

impl TweetStore {
    const PREFIX: &str = "net.bouzuya.rust-sandbox.twiq-light";
    const KEY: &str = "tweet.json";

    pub async fn read_all(&self) -> anyhow::Result<BTreeMap<String, MyTweet>> {
        let item = self.storage.get_item(PathBuf::from(Self::KEY)).await?;
        Ok(match item {
            None => BTreeMap::default(),
            Some(s) => serde_json::from_str(&s)?,
        })
    }

    pub async fn write_all(&self, data: &BTreeMap<String, MyTweet>) -> anyhow::Result<()> {
        self.storage
            .set_item(PathBuf::from(Self::KEY), serde_json::to_string(data)?)
            .await
    }
}
