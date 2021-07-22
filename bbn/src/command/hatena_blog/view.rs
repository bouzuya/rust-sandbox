use anyhow::Context;
use chrono::{DateTime, Local, TimeZone};
use date_range::date::Date;

use crate::{
    bbn_repository::BbnRepository, config_repository::ConfigRepository,
    hatena_blog::HatenaBlogRepository,
};

pub async fn view(
    content: bool,
    date: Date,
    hatena_blog_id: String,
    meta: bool,
    web: bool,
) -> anyhow::Result<()> {
    let config_repository = ConfigRepository::new();
    let config = config_repository
        .load()
        .context("The configuration file does not found. Use `bbn config` command.")?;
    let data_dir = config.data_dir().to_path_buf();
    let data_file = config.hatena_blog_data_file().to_path_buf();

    let hatena_blog_repository = HatenaBlogRepository::new(data_file).await?;
    let bbn_repository = BbnRepository::new(data_dir.clone());

    let bbn_entry_id = bbn_repository
        .find_id_by_date(date)?
        .context("no entry_id")?;
    let bbn_entry_meta = bbn_repository
        .find_meta_by_id(&bbn_entry_id)?
        .context("no entry_meta")?;
    let hatena_blog_entry = hatena_blog_repository
        .find_entry_by_updated(bbn_entry_meta.pubdate.into())
        .await?
        .context("no hatena-blog entry")?;
    let updated = DateTime::parse_from_rfc3339(&hatena_blog_entry.updated.to_string())?;
    // TODO: get offset from options
    let local = Local.from_utc_datetime(&updated.naive_utc());
    let url = format!(
        "https://{}/entry/{}",
        hatena_blog_id,
        local.format("%Y/%m/%d/%H%M%S")
    );
    if web {
        open::that(url)?;
    } else {
        match (content, meta) {
            (false, false) | (true, false) => {
                println!("{}", hatena_blog_entry.content);
            }
            (false, true) => {
                println!(
                    "{} {} <{}>",
                    bbn_entry_meta.pubdate, hatena_blog_entry.title, url,
                );
            }
            (true, true) => {
                println!(
                    "{} {} <{}>\n{}",
                    bbn_entry_meta.pubdate, hatena_blog_entry.title, url, hatena_blog_entry.content
                );
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // TODO
}
