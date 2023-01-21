use anyhow::Context;

use crate::config_repository::ConfigRepository;
use bbn_hatena_blog::HatenaBlogRepository;
use bbn_repository::{BbnRepository, Query};
use std::convert::TryFrom;

pub async fn diff(date: Option<String>) -> anyhow::Result<()> {
    let config_repository = ConfigRepository::new()?;
    let config = config_repository
        .load()
        .context("The configuration file does not found. Use `bbn config` command.")?;
    let data_dir = config.data_dir().to_path_buf();
    let data_file = config.hatena_blog_data_file().to_path_buf();

    let hatena_blog_repository = HatenaBlogRepository::new(data_file).await?;
    let bbn_repository = BbnRepository::new(data_dir.clone());

    let query_string = match date {
        Some(ref s) => format!("date:{}", s),
        None => "".to_string(),
    };
    let query = Query::try_from(query_string.as_str())?;
    let entry_ids = bbn_repository.find_ids_by_query(query)?;
    let mut diff_stats = (0, 0, 0, 0);
    for entry_id in entry_ids {
        let bbn_entry = bbn_repository.find_entry_by_id(&entry_id)?.unwrap();
        if bbn_entry.meta().hatena_blog_ignore == Some(true) {
            diff_stats.3 += 1;
            continue;
        }
        let hatena_blog_entry = hatena_blog_repository
            .find_entry_by_entry_meta(bbn_entry.meta())
            .await?;
        let result = hatena_blog_entry
            .as_ref()
            .map(|entry| bbn_entry.content() == entry.content);
        match result {
            None => diff_stats.0 += 1,
            Some(false) => diff_stats.2 += 1,
            Some(true) => diff_stats.1 += 1,
        }
        if result != Some(true) {
            if date.is_none() {
                println!(
                    "{} {}",
                    result.map(|b| if b { "eq" } else { "ne" }).unwrap_or("no"),
                    entry_id
                );
            } else if let Some(entry) = hatena_blog_entry {
                show_diff(entry.content.as_str(), bbn_entry.content());
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

fn show_stats(diff_stats: (i32, i32, i32, i32)) {
    println!(
        "different count: eq = {} ne = {} no = {} ig = {} (ne + no = {})",
        diff_stats.1,
        diff_stats.2,
        diff_stats.0,
        diff_stats.3,
        diff_stats.2 + diff_stats.0,
    );
}
