use crate::config_repository::ConfigRepository;
use anyhow::Context;
use bbn_data::{EntryId, EntryMeta};
use bbn_repository::BbnRepository;
use date_range::date::Date;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ContentJson {
    content: String,
    url: String,
}

#[derive(Debug, Serialize)]
struct ContentWithMetaJson {
    content: String,
    minutes: u64,
    pubdate: String,
    tags: Vec<String>,
    title: String,
    url: String,
}

#[derive(Debug, Serialize)]
struct MetaJson {
    minutes: u64,
    pubdate: String,
    tags: Vec<String>,
    title: String,
    url: String,
}

fn print_json_content(_: EntryId, _: EntryMeta, entry_content: String) -> anyhow::Result<()> {
    println!("{}", serde_json::to_string(&entry_content)?);
    Ok(())
}

fn print_json_meta(entry_id: EntryId, entry_meta: EntryMeta, _: String) -> anyhow::Result<()> {
    println!(
        "{}",
        serde_json::to_string(&MetaJson {
            minutes: entry_meta.minutes,
            pubdate: entry_meta.pubdate.to_string(),
            tags: entry_meta.tags,
            title: entry_meta.title,
            url: entry_url(&entry_id)
        })?
    );
    Ok(())
}

fn print_json_content_meta(
    entry_id: EntryId,
    entry_meta: EntryMeta,
    entry_content: String,
) -> anyhow::Result<()> {
    println!(
        "{}",
        serde_json::to_string(&ContentWithMetaJson {
            content: entry_content,
            minutes: entry_meta.minutes,
            pubdate: entry_meta.pubdate.to_string(),
            tags: entry_meta.tags,
            title: entry_meta.title,
            url: entry_url(&entry_id)
        })?
    );
    Ok(())
}

fn print_text_content(_: EntryId, _: EntryMeta, entry_content: String) -> anyhow::Result<()> {
    println!("{}", entry_content);
    Ok(())
}

fn print_text_content_meta(
    entry_id: EntryId,
    entry_meta: EntryMeta,
    entry_content: String,
) -> anyhow::Result<()> {
    println!(
        "{}{} {} <{}>\n{}",
        entry_id.date(),
        entry_id
            .id_title()
            .map(|s| format!(" {}", s))
            .unwrap_or_default(),
        entry_meta.title,
        entry_url(&entry_id),
        entry_content
    );
    Ok(())
}

fn print_text_meta(entry_id: EntryId, entry_meta: EntryMeta, _: String) -> anyhow::Result<()> {
    println!(
        "{}{} {} <{}>",
        entry_id.date(),
        entry_id
            .id_title()
            .map(|s| format!(" {}", s))
            .unwrap_or_default(),
        entry_meta.title,
        entry_url(&entry_id)
    );
    Ok(())
}

fn entry_url(entry_id: &EntryId) -> String {
    format!(
        "https://blog.bouzuya.net/{}/",
        entry_id.date().to_string().replace('-', "/")
    )
}

pub fn view(date: Date, content: bool, json: bool, meta: bool, web: bool) -> anyhow::Result<()> {
    let config_repository = ConfigRepository::new()?;
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
            entry.map(|entry| (entry_id, entry.meta().clone(), entry.content().to_string()))
        })
        .context("not found")?;
    if web {
        open::that(entry_url(&entry_id))?;
        return Ok(());
    }
    let print = match (content, meta, json) {
        (false, true, false) => print_text_meta,
        (false, false, false) | (true, false, false) => print_text_content,
        (true, true, false) => print_text_content_meta,
        (false, true, true) => print_json_meta,
        (false, false, true) | (true, false, true) => print_json_content,
        (true, true, true) => print_json_content_meta,
    };
    print(entry_id, entry_meta, entry_content)
}
