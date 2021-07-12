use crate::{bbn_repository::BbnRepository, config_repository::ConfigRepository};
use anyhow::Context;
use date_range::date::Date;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ContentJson {
    content: String,
}

#[derive(Debug, Serialize)]
struct ContentWithMetaJson {
    content: String,
    minutes: u64,
    pubdate: String,
    tags: Vec<String>,
    title: String,
}

#[derive(Debug, Serialize)]
struct MetaJson {
    minutes: u64,
    pubdate: String,
    tags: Vec<String>,
    title: String,
}

pub fn view(date: Date, content: bool, json: bool, meta: bool, web: bool) -> anyhow::Result<()> {
    let config_repository = ConfigRepository::new();
    let config = config_repository
        .load()
        .context("The configuration file does not found. Use `bbn config` command.")?;
    let data_dir = config.data_dir().to_path_buf();

    let repository = BbnRepository::new(data_dir);
    let entry_id = repository.find_id_by_date(date)?;
    let entry = entry_id
        .as_ref()
        .and_then(|entry_id| repository.find_entry_by_id(entry_id).transpose())
        .transpose()?;
    let (entry_id, entry_meta, entry_content) = entry_id
        .and_then(|entry_id| {
            entry.map(|(entry_meta, entry_content)| (entry_id, entry_meta, entry_content))
        })
        .context("not found")?;
    let url = format!(
        "https://blog.bouzuya.net/{}/",
        entry_id.date().to_string().replace('-', "/")
    );
    if web {
        open::that(url)?;
        return Ok(());
    }
    if json {
        println!(
            "{}",
            match (content, meta) {
                (true, true) => {
                    serde_json::to_string(&ContentWithMetaJson {
                        content: entry_content,
                        minutes: entry_meta.minutes,
                        pubdate: entry_meta.pubdate.to_rfc3339(),
                        tags: entry_meta.tags,
                        title: entry_meta.title,
                    })?
                }
                (true, false) | (false, false) => {
                    serde_json::to_string(&ContentJson {
                        content: entry_content,
                    })?
                }
                (false, true) => {
                    serde_json::to_string(&MetaJson {
                        minutes: entry_meta.minutes,
                        pubdate: entry_meta.pubdate.to_rfc3339(),
                        tags: entry_meta.tags,
                        title: entry_meta.title,
                    })?
                }
            }
        );
    } else {
        if meta {
            println!(
                "{}{} {} <{}>",
                entry_id.date(),
                entry_id
                    .id_title()
                    .map(|s| format!(" {}", s))
                    .unwrap_or_default(),
                entry_meta.title,
                url
            );
        }
        if content {
            println!("{}", entry_content);
        }
    }
    Ok(())
}
