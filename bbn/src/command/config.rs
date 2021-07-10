use std::path::PathBuf;

use anyhow::Context;

use crate::config_repository::{Config, ConfigRepository};

pub fn config(data_dir: PathBuf) -> anyhow::Result<()> {
    let config_repository = ConfigRepository::new();
    let config = Config::new(data_dir);
    config_repository.save(config)?;
    println!(
        "The configuration has been written to {}",
        config_repository
            .path()?
            .to_str()
            .context("The configuration file path is not UTF-8")?
    );
    Ok(())
}
