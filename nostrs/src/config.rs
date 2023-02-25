use std::{collections::HashMap, fs::File, io::BufReader};

use crate::dirs::config_dir;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    // NIP-46
    pub relays: HashMap<String, RelayOptions>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RelayOptions {
    pub read: bool,
    pub write: bool,
}

pub fn load() -> anyhow::Result<Config> {
    let path = config_dir()?.join("config.json");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader)?;
    Ok(config)
}
