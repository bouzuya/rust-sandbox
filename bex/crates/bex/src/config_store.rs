use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::store::Store;

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Config {
    pub consumer_key: String,
}

pub struct ConfigStore {
    path: PathBuf,
}

impl ConfigStore {
    pub fn new<P: AsRef<Path>>(dir: P) -> Self {
        Self {
            path: dir.as_ref().join("config.json"),
        }
    }
}

impl Store for ConfigStore {
    type Item = Config;

    fn path(&self) -> &Path {
        self.path.as_path()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let config_store = ConfigStore::new(temp_dir.path());

        assert_eq!(config_store.path(), temp_dir.path().join("config.json"));
        assert_eq!(config_store.load()?, None);
        let config = Config {
            consumer_key: "123456-0123456789abcdef0123456".to_string(),
        };
        config_store.save(&config)?;
        assert_eq!(config_store.load()?, Some(config));
        config_store.delete()?;
        assert_eq!(config_store.load()?, None);

        Ok(())
    }
}
