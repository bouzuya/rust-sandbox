use anyhow::Context;
use chrono::{DateTime, Local, TimeZone};
use date_range::date::Date;

use crate::{bbn_hatena_blog::BbnHatenaBlogRepository, bbn_repository::BbnRepository};
use std::path::PathBuf;

pub async fn view(
    data_dir: PathBuf,
    data_file: PathBuf,
    date: Date,
    hatena_blog_id: String,
    web: bool,
) -> anyhow::Result<()> {
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
    if web {
        let updated = DateTime::parse_from_rfc3339(hatena_blog_entry.updated.as_str())?;
        // TODO: get offset from options
        let local = Local.from_utc_datetime(&updated.naive_utc());
        let url = format!(
            "https://{}/entry/{}",
            hatena_blog_id,
            local.format("%Y/%m/%d/%H%M%S")
        );
        open::that(url)?;
    } else {
        println!("{}", hatena_blog_entry.content);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // TODO
}
