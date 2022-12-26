use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
};

use crate::domain::MyTweet;

#[derive(Debug)]
pub struct TweetStore {
    path: PathBuf,
}

impl Default for TweetStore {
    fn default() -> Self {
        let path = Path::new(&env::var("HOME").expect("env HOME")).join("twiq-light.json");
        Self { path }
    }
}

impl TweetStore {
    pub fn read_all(&self) -> anyhow::Result<BTreeMap<String, MyTweet>> {
        if !self.path().exists() {
            Ok(BTreeMap::new())
        } else {
            let s = fs::read_to_string(self.path())?;
            Ok(serde_json::from_str(&s)?)
        }
    }

    pub fn write_all(&self, data: &BTreeMap<String, MyTweet>) -> anyhow::Result<()> {
        Ok(fs::write(self.path(), serde_json::to_string(data)?)?)
    }

    fn path(&self) -> &Path {
        &self.path
    }
}
