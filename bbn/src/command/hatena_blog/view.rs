use anyhow::Context;
use date_range::date::Date;

use crate::{bbn_hatena_blog::BbnHatenaBlogRepository, bbn_repository::BbnRepository};
use std::path::PathBuf;

pub async fn view(data_dir: PathBuf, data_file: PathBuf, date: Date) -> anyhow::Result<()> {
    let hatena_blog_repository = BbnHatenaBlogRepository::new(data_file).await?;
    let bbn_repository = BbnRepository::new(data_dir.clone());

    let bbn_entry_id = bbn_repository
        .find_id_by_date(date)?
        .context("no entry_id")?;
    let bbn_entry_meta = bbn_repository
        .find_meta_by_id(&bbn_entry_id)?
        .context("no entry_meta")?;
    let hatena_blog_entry = hatena_blog_repository
        .find_entry_by_updated(bbn_entry_meta.pubdate)
        .await?
        .context("no hatena-blog entry")?;
    println!("{}", hatena_blog_entry.content);

    Ok(())
}

#[cfg(test)]
mod tests {
    // TODO
}
