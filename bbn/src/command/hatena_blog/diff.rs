use anyhow::Context;
use date_range::date::Date;
use hatena_blog::{Entry, GetEntryResponse};
use serde_json::Value;

use crate::{bbn_hatena_blog::Repository, post::list_posts, query::Query, timestamp::Timestamp};
use std::{
    convert::TryFrom,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

async fn parse_entry(repository: &Repository) -> anyhow::Result<()> {
    for (entry_id, body) in repository.find_entries_waiting_for_parsing().await? {
        let entry = Entry::try_from(GetEntryResponse::from(body))?;
        repository.create_entry(entry).await?;
        eprintln!("{}", entry_id);
    }
    Ok(())
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct BbnEntry {
    content: String,
    date: Date,
    id_title: Option<String>,
    minutes: u64,
    pubdate: Timestamp,
    title: String,
    // TODO: tags: Vec<String>
}

fn get_bbn_entry(
    data_dir: &Path,
    date: Date,
    id_title: Option<String>,
) -> anyhow::Result<BbnEntry> {
    let dir = data_dir
        .join(date.year().to_string())
        .join(date.month().to_string());
    let id_title_suffix = id_title
        .clone()
        .map(|s| format!("-{}", s))
        .unwrap_or_default();
    let content = fs::read_to_string(dir.join(format!("{}{}.md", date, id_title_suffix)))?;
    let json_content = fs::read_to_string(dir.join(format!("{}{}.json", date, id_title_suffix)))?;
    let json: Value = serde_json::from_str(&json_content)?;
    let minutes = json
        .get("minutes")
        .context("get minutes")?
        .as_u64()
        .context("parse minutes")?;
    let pubdate = Timestamp::from_rfc3339(
        &json
            .get("pubdate")
            .context("get pubdate")?
            .as_str()
            .context("parse pubdate")?
            .to_string(),
    )?;
    let title = json
        .get("title")
        .context("get title")?
        .as_str()
        .context("parse title")?
        .to_string();
    Ok(BbnEntry {
        content,
        date,
        id_title,
        minutes,
        pubdate,
        title,
    })
}

pub async fn diff(data_dir: PathBuf, data_file: PathBuf) -> anyhow::Result<()> {
    let repository = Repository::new(data_file).await?;

    parse_entry(&repository).await?;

    let mut diff_stats = (0, 0, 0);
    let posts = list_posts(data_dir.as_path(), &Query::try_from("")?)?;
    for post in posts {
        let date = Date::from_str(post.date.as_str()).unwrap();
        let bbn_entry = get_bbn_entry(data_dir.as_path(), date, post.id_title.clone())?;

        let entry = repository.find_entry_by_updated(bbn_entry.pubdate).await?;
        let result = match entry {
            None => None,
            Some(entry) => {
                if bbn_entry.content != entry.content {
                    Some(false)
                } else {
                    Some(true)
                }
            }
        };
        match result {
            None => diff_stats.0 += 1,
            Some(false) => diff_stats.1 += 1,
            Some(true) => diff_stats.2 += 1,
        }
        if result != Some(true) {
            println!(
                "{} {:?}",
                result.map(|b| if b { "eq" } else { "ne" }).unwrap_or("no"),
                post
            );
        }
    }
    println!(
        "different count: eq = {} ne = {} no = {} (ne + no = {})",
        diff_stats.1,
        diff_stats.2,
        diff_stats.0,
        diff_stats.2 + diff_stats.0,
    );

    Ok(())
}
