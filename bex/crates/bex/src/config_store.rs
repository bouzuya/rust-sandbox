use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::store::Store;

#[derive(Debug, Deserialize, Serialize)]
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
    #[test]
    fn test() {
        // TODO
    }
}
