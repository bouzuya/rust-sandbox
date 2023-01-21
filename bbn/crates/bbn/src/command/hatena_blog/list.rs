use anyhow::Context;

use crate::config_repository::ConfigRepository;
use bbn_hatena_blog::HatenaBlogRepository;

pub async fn list() -> anyhow::Result<()> {
    let config_repository = ConfigRepository::new()?;
    let config = config_repository
        .load()
        .context("The configuration file does not found. Use `bbn config` command.")?;
    let data_file = config.hatena_blog_data_file().to_path_buf();

    let hatena_blog_repository = HatenaBlogRepository::new(data_file).await?;

    for (updated, title) in hatena_blog_repository
        .find_entries_updated_and_title()
        .await?
    {
        println!("{} {}", updated.to_rfc3339(), title);
    }

    Ok(())
}
