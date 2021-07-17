use anyhow::Context;
use date_range::date::Date;
use hatena_blog::{Client, Config};

use crate::{
    bbn_repository::BbnRepository,
    config_repository::ConfigRepository,
    hatena_blog::{upload_entry, HatenaBlogRepository},
};

pub async fn upload(
    date: Date,
    draft: bool,
    hatena_api_key: String,
    hatena_blog_id: String,
    hatena_id: String,
) -> anyhow::Result<()> {
    let config_repository = ConfigRepository::new();
    let config = config_repository
        .load()
        .context("The configuration file does not found. Use `bbn config` command.")?;
    let data_dir = config.data_dir().to_path_buf();
    let bbn_repository = BbnRepository::new(data_dir);
    let hatena_blog_data_file = config.hatena_blog_data_file().to_path_buf();
    let hatena_blog_repository = HatenaBlogRepository::new(hatena_blog_data_file).await?;
    let config = Config::new(
        hatena_id.as_str(),
        None,
        hatena_blog_id.as_str(),
        hatena_api_key.as_str(),
    );
    let hatena_blog_client = Client::new(&config);
    let (created, entry_id) = upload_entry(
        date,
        draft,
        hatena_id,
        bbn_repository,
        hatena_blog_repository,
        hatena_blog_client,
    )
    .await?;
    println!(
        "{} {}",
        if created { "created" } else { "updated" },
        entry_id
    );
    Ok(())
}
