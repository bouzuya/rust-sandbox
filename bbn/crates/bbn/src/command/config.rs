use std::path::PathBuf;

use anyhow::Context;

use crate::{config::Config, config_repository::ConfigRepository};

pub fn config(data_dir: PathBuf, hatena_blog_data_file: PathBuf) -> anyhow::Result<()> {
    // FIXME: Add argument to add link_completion_rules_file
    let config_repository = ConfigRepository::new()?;
    let config = Config::new(data_dir, hatena_blog_data_file, None);
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
