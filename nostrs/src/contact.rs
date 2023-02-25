use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
};

use nostr_sdk::prelude::{Metadata, Timestamp, XOnlyPublicKey};

use crate::dirs;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Contacts {
    pub contacts: HashMap<XOnlyPublicKey, Option<Metadata>>,
    pub updated_at: Option<Timestamp>,
}

pub fn load() -> anyhow::Result<Contacts> {
    let path = dirs::cache_dir()?.join("contacts.json");
    if path.exists() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let contacts = serde_json::from_reader(reader)?;
        Ok(contacts)
    } else {
        Ok(Default::default())
    }
}

pub fn store(contacts: &Contacts) -> anyhow::Result<()> {
    let path = dirs::cache_dir()?.join("contacts.json");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string(contacts)?;
    fs::write(path, content)?;
    Ok(())
}
