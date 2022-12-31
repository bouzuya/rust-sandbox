use std::{env, path::PathBuf};

use crate::{
    data::Config,
    storage::{fs::FsStorage, Storage},
};

#[derive(Debug)]
pub struct ConfigStore {
    storage: FsStorage,
}

impl Default for ConfigStore {
    fn default() -> Self {
        let storage = FsStorage::new(PathBuf::from(&env::var("HOME").expect("env HOME")));
        Self { storage }
    }
}

impl ConfigStore {
    const KEY: &str = "twiq-light-config.json";

    pub async fn read(&self) -> anyhow::Result<Option<Config>> {
        let item = self.storage.get_item(PathBuf::from(Self::KEY)).await?;
        Ok(match item {
            None => None,
            Some(s) => Some(serde_json::from_str::<'_, Config>(&s)?),
        })
    }

    pub async fn write(&self, data: &Config) -> anyhow::Result<()> {
        self.storage
            .set_item(PathBuf::from(Self::KEY), serde_json::to_string(data)?)
            .await
    }
}
