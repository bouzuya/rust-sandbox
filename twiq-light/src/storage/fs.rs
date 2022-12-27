use std::{fs, path::PathBuf};

use async_trait::async_trait;

use super::Storage;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FsStorage {
    root: PathBuf,
}

impl FsStorage {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
}

#[async_trait]
impl Storage for FsStorage {
    type Key = PathBuf;

    type Value = String;

    async fn get_item(&self, key: Self::Key) -> anyhow::Result<Option<Self::Value>> {
        let path = self.root.join(key);
        if !path.exists() {
            return Ok(None);
        }
        Ok(Some(fs::read_to_string(path)?))
    }

    async fn keys(&self) -> anyhow::Result<Vec<Self::Key>> {
        let mut keys = vec![];
        let read_dir = fs::read_dir(self.root.as_path())?;
        for dir_entry in read_dir {
            let dir_entry = dir_entry?;
            let path = dir_entry.path();
            let key = path.strip_prefix(&self.root)?.to_owned();
            keys.push(key);
        }
        Ok(keys)
    }

    async fn remove_item(&self, key: Self::Key) -> anyhow::Result<()> {
        let path = self.root.join(key);
        if !path.exists() {
            return Ok(());
        }
        Ok(fs::remove_file(path)?)
    }

    async fn set_item(&self, key: Self::Key, value: Self::Value) -> anyhow::Result<()> {
        let path = self.root.join(key);
        Ok(fs::write(path, value)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let root = temp_dir.path().join("root");
        fs::create_dir_all(&root)?;
        let storage = FsStorage::new(root);

        let key1: PathBuf = "key1".into();
        let val1: String = "value1".to_owned();

        assert!(storage.keys().await?.is_empty());
        assert_eq!(storage.get_item(key1.clone()).await?, None);

        storage.set_item(key1.clone(), val1.clone()).await?;
        assert_eq!(storage.keys().await?, vec![key1.clone()]);
        assert_eq!(storage.get_item(key1.clone()).await?, Some(val1));

        storage.remove_item(key1.clone()).await?;
        assert!(storage.keys().await?.is_empty());
        assert_eq!(storage.get_item(key1.clone()).await?, None);
        Ok(())
    }
}
