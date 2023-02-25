use std::{env, path::PathBuf};

use xdg::BaseDirectories;

pub fn config_dir() -> anyhow::Result<PathBuf> {
    let prefix = "net.bouzuya.rust-sandbox.nostrs";
    Ok(match env::var_os("NOSTRS_CONFIG_DIR") {
        Some(config_dir) => PathBuf::from(config_dir),
        None => BaseDirectories::with_prefix(prefix)?.get_config_home(),
    })
}

pub fn state_dir() -> anyhow::Result<PathBuf> {
    let prefix = "net.bouzuya.rust-sandbox.nostrs";
    Ok(match env::var_os("NOSTRS_STATE_DIR") {
        Some(state_dir) => PathBuf::from(state_dir),
        None => BaseDirectories::with_prefix(prefix)?.get_state_home(),
    })
}
