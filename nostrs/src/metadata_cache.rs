use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
    time::Duration,
};

use nostr_sdk::prelude::{Metadata, Timestamp, XOnlyPublicKey};

use crate::dirs;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct MetadataCache(HashMap<XOnlyPublicKey, MetadataCacheItem>);

impl MetadataCache {
    pub fn get(&self, key: XOnlyPublicKey) -> Option<Metadata> {
        self.0.get(&key).cloned().and_then(|item| {
            if item.updated_at < Timestamp::now() - Duration::from_secs(60 * 60) {
                None
            } else {
                Some(item.metadata)
            }
        })
    }

    pub fn set(&mut self, key: XOnlyPublicKey, metadata: Metadata) {
        self.0.insert(
            key,
            MetadataCacheItem {
                metadata,
                updated_at: Timestamp::now(),
            },
        );
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct MetadataCacheItem {
    pub metadata: Metadata,
    pub updated_at: Timestamp,
}

pub fn load() -> anyhow::Result<MetadataCache> {
    let path = dirs::cache_dir()?.join("metadata.json");
    if path.exists() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let metadata_cache = serde_json::from_reader(reader)?;
        Ok(metadata_cache)
    } else {
        Ok(Default::default())
    }
}

pub fn store(metadata_cache: &MetadataCache) -> anyhow::Result<()> {
    let path = dirs::cache_dir()?.join("metadata.json");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string(metadata_cache)?;
    fs::write(path, content)?;
    Ok(())
}
