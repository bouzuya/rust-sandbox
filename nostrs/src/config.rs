use std::{collections::HashMap, env, fs::File, io::BufReader, path::PathBuf};

use xdg::BaseDirectories;

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

fn config_dir() -> anyhow::Result<PathBuf> {
    let prefix = "net.bouzuya.rust-sandbox.nostrs";
    Ok(match env::var_os("NOSTRS_CONFIG_DIR") {
        Some(config_dir) => PathBuf::from(config_dir),
        None => BaseDirectories::with_prefix(prefix)?.get_config_home(),
    })
}
