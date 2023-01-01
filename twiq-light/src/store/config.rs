use std::{env, path::PathBuf};

use xdg::BaseDirectories;

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
        let config_dir = match env::var_os("TWIQ_LIGHT_CONFIG_DIR") {
            None => BaseDirectories::with_prefix(Self::PREFIX)
                .expect("xdg")
                .get_config_home(),
            Some(config_dir) => PathBuf::from(config_dir),
        };
        let storage = FsStorage::new(config_dir);
        Self { storage }
    }
}

impl ConfigStore {
    const PREFIX: &str = "net.bouzuya.rust-sandbox.twiq-light";
    const KEY: &str = "config.json";

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
