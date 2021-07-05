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

pub async fn diff(
    data_dir: PathBuf,
    data_file: PathBuf,
    date: Option<String>,
) -> anyhow::Result<()> {
    let repository = Repository::new(data_file).await?;

    parse_entry(&repository).await?;

    let query_string = match date {
        Some(ref s) => format!("date:{}", s),
        None => "".to_string(),
    };
    let query = Query::try_from(query_string.as_str())?;
    let posts = list_posts(data_dir.as_path(), &query)?;
    let mut diff_stats = (0, 0, 0);
    for post in posts {
        let entry_date = Date::from_str(post.date.as_str()).unwrap();
        let bbn_entry = get_bbn_entry(data_dir.as_path(), entry_date, post.id_title.clone())?;

        let entry = repository.find_entry_by_updated(bbn_entry.pubdate).await?;
        let result = match entry {
            None => None,
            Some(ref entry) => {
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
            if date.is_none() {
                println!(
                    "{} {:?}",
                    result.map(|b| if b { "eq" } else { "ne" }).unwrap_or("no"),
                    post
                );
            } else if let Some(entry) = entry {
                show_diff(entry.content.as_str(), bbn_entry.content.as_str());
            }
        }
    }
    if date.is_none() {
        show_stats(diff_stats);
    }

    Ok(())
}

fn show_diff(left: &str, right: &str) {
    for diff_result in diff::lines(left, right) {
        println!(
            "{}",
            match diff_result {
                diff::Result::Left(l) => console::style(format!("-{}", l)).red(),
                diff::Result::Both(l, _) => console::style(format!(" {}", l)),
                diff::Result::Right(r) => console::style(format!("+{}", r)).green(),
            }
        );
    }
}

fn show_stats(diff_stats: (i32, i32, i32)) {
    println!(
        "different count: eq = {} ne = {} no = {} (ne + no = {})",
        diff_stats.1,
        diff_stats.2,
        diff_stats.0,
        diff_stats.2 + diff_stats.0,
    );
}
