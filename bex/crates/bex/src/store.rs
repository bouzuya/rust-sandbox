use std::{fs, path::Path};

use serde::{de::DeserializeOwned, Serialize};

pub trait Store {
    type Item: DeserializeOwned + Serialize;

    fn delete(&self) -> anyhow::Result<()> {
        let p = self.path();
        if p.exists() {
            fs::remove_file(p)?;
        }
        Ok(())
    }

    fn load(&self) -> anyhow::Result<Option<Self::Item>> {
        let p = self.path();
        if p.exists() {
            let s = fs::read_to_string(p)?;
            Ok(Some(serde_json::from_str(&s)?))
        } else {
            Ok(None)
        }
    }

    fn path(&self) -> &Path;

    fn save(&self, item: &Self::Item) -> anyhow::Result<()> {
        let p = self.path();
        if let Some(dir) = p.parent() {
            fs::create_dir_all(dir)?;
        }
        fs::write(p, serde_json::to_string(item)?)?;
        Ok(())
    }
}
