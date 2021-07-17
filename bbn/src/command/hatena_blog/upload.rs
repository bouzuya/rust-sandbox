use anyhow::Context;
use date_range::date::Date;
use hatena_blog::{Client, Config, EntryParams};

use crate::{
    bbn_repository::BbnRepository, config_repository::ConfigRepository,
    hatena_blog::BbnHatenaBlogRepository,
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
    let hatena_blog_repository = BbnHatenaBlogRepository::new(hatena_blog_data_file).await?;
    let config = Config::new(
        hatena_id.as_str(),
        None,
        hatena_blog_id.as_str(),
        hatena_api_key.as_str(),
    );
    let hatena_blog_client = Client::new(&config);
    upload_impl(
        date,
        draft,
        hatena_id,
        bbn_repository,
        hatena_blog_repository,
        hatena_blog_client,
    )
    .await
}

async fn upload_impl(
    date: Date,
    draft: bool,
    hatena_id: String,
    bbn_repository: BbnRepository,
    hatena_blog_repository: BbnHatenaBlogRepository,
    hatena_blog_client: Client,
) -> anyhow::Result<()> {
    let entry_id = bbn_repository
        .find_id_by_date(date)?
        .context("entry id not found")?;
    let (entry_meta, entry_content) = bbn_repository
        .find_entry_by_id(&entry_id)?
        .context("entry not found")?;
    let updated = entry_meta.pubdate;
    let params = EntryParams::new(
        hatena_id,
        entry_meta.title.clone(),
        entry_content,
        updated.to_rfc3339(),
        vec![],
        draft,
    );
    match hatena_blog_repository
        .find_entry_by_updated(updated)
        .await?
    {
        None => {
            hatena_blog_client.create_entry(params).await?;
            println!("create {} {}", date, entry_meta.title);
        }
        Some(entry) => {
            hatena_blog_client.update_entry(&entry.id, params).await?;
            println!("update {} {}", date, entry_meta.title);
        }
    }
    Ok(())
}
