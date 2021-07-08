use std::{path::PathBuf, str::FromStr};

use anyhow::Context;
use date_range::date::Date;
use hatena_blog::{Client, Config, EntryParams};

use crate::bbn_repository::BbnRepository;

pub async fn post_to_hatena_blog(
    data_dir: PathBuf,
    date: String,
    draft: bool,
    hatena_api_key: String,
    hatena_blog_id: String,
    hatena_id: String,
) -> anyhow::Result<()> {
    let bbn_repository = BbnRepository::new(data_dir);
    let entry_id = bbn_repository.find_id_by_date(Date::from_str(date.as_str())?)?;
    let entry_id = entry_id.context("id not found")?;
    let (entry_meta, entry_content) = bbn_repository
        .find_entry_by_id(&entry_id)?
        .context("not found")?;
    let config = Config::new(
        hatena_id.as_str(),
        None,
        hatena_blog_id.as_str(),
        hatena_api_key.as_str(),
    );
    let client = Client::new(&config);
    client
        .create_entry(EntryParams::new(
            hatena_id,
            entry_meta.title.clone(),
            entry_content,
            entry_meta.pubdate.to_rfc3339(),
            vec![],
            draft,
        ))
        .await?;
    println!("{} {}", date, entry_meta.title);
    Ok(())
}
