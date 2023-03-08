use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
    time::Duration,
};

use nostr_sdk::prelude::{Metadata, Timestamp, XOnlyPublicKey};

use crate::{client::Client, dirs};

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct MetadataCache(HashMap<XOnlyPublicKey, MetadataCacheItem>);

impl MetadataCache {
    pub fn get(&self, key: XOnlyPublicKey) -> Option<MetadataCacheItem> {
        self.0.get(&key).cloned()
    }

    pub fn set(&mut self, key: XOnlyPublicKey, item: MetadataCacheItem) {
        self.0.insert(key, item);
    }

    pub async fn update(
        &mut self,
        key: XOnlyPublicKey,
        client: &Client,
    ) -> anyhow::Result<Option<Metadata>> {
        Ok(match self.get(key) {
            Some(MetadataCacheItem {
                metadata,
                updated_at,
            }) if updated_at < Timestamp::now() - Duration::from_secs(60 * 60) => Some(metadata),
            Some(MetadataCacheItem {
                metadata,
                updated_at,
            }) => {
                let new_metadata = client
                    .get_metadata(key, Some(updated_at))
                    .await?
                    .or(Some(metadata));
                if let Some(metadata) = new_metadata.clone() {
                    self.set(
                        key,
                        MetadataCacheItem {
                            metadata,
                            updated_at: Timestamp::now(),
                        },
                    );
                }
                new_metadata
            }
            None => {
                let metadata = client.get_metadata(key, None).await?;
                if let Some(metadata) = metadata.clone() {
                    self.set(
                        key,
                        MetadataCacheItem {
                            metadata,
                            updated_at: Timestamp::now(),
                        },
                    );
                }
                metadata
            }
        })
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
