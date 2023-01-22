use anyhow::Context;
use bbn_repository::{BbnRepository, Query};
use std::{
    collections::BTreeMap,
    convert::TryFrom,
    fs::{self, File},
    io::BufWriter,
    path::{Path, PathBuf},
};

use crate::config_repository::ConfigRepository;

// <https://github.com/bouzuya/kraken/tree/v4.0.2/doc#all-json>
// all json (`/posts.json`)
#[derive(serde::Serialize)]
pub struct AllJson(Vec<AllJsonItem>);

#[derive(serde::Serialize)]
pub struct AllJsonItem {
    pub date: String,             // "YYYY-MM-DD"
    pub id_title: Option<String>, // "title" (obsolete)
    pub minutes: u32,
    pub pubdate: String, // "YYYY-MM-DDTHH:MM:SSZ"
    pub tags: Vec<String>,
    pub title: String,
}

// <https://github.com/bouzuya/kraken/tree/v4.0.2/doc#tags-json>
// tags json (`/tags.json`)
#[derive(serde::Serialize)]
pub struct TagsJson(Vec<TagsJsonItem>);

#[derive(serde::Serialize)]
pub struct TagsJsonItem {
    pub name: String,
    pub count: u32,
}

fn write_all_json(out_dir: &Path, all_json: &AllJson) -> anyhow::Result<()> {
    let path = out_dir.join("posts.json");
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, all_json)?;
    Ok(())
}

fn write_tags_json(out_dir: &Path, tags_json: &TagsJson) -> anyhow::Result<()> {
    let path = out_dir.join("tags.json");
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, tags_json)?;
    Ok(())
}

pub fn run(out_dir: PathBuf) -> anyhow::Result<()> {
    let config_repository = ConfigRepository::new()?;
    let config = config_repository
        .load()
        .context("The configuration file does not found. Use `bbn config` command.")?;
    let data_dir = config.data_dir().to_path_buf();

    let bbn_repository = BbnRepository::new(data_dir);
    let query = Query::try_from("date:1970-01-01/9999-12-31")?;
    let mut entry_ids = bbn_repository.find_ids_by_query(query)?;
    entry_ids.sort();

    let mut all_json_items = vec![];
    let mut tag_count_map = BTreeMap::new();
    for entry_id in entry_ids {
        let meta = bbn_repository
            .find_meta_by_id(&entry_id)?
            .context("meta not found")?;

        for name in meta.tags.clone() {
            *tag_count_map.entry(name).or_insert(0) += 1;
        }

        let all_json_item = AllJsonItem {
            date: entry_id.date().to_string(),
            id_title: entry_id.id_title().map(|s| s.to_owned()),
            minutes: u32::try_from(meta.minutes)?,
            pubdate: meta.pubdate.to_string(),
            tags: meta.tags,
            title: meta.title,
        };
        all_json_items.push(all_json_item);
    }

    let tags_json = TagsJson(
        tag_count_map
            .into_iter()
            .map(|(name, count)| TagsJsonItem { name, count })
            .collect::<Vec<_>>(),
    );

    let all_json = AllJson(all_json_items);

    fs::create_dir_all(out_dir.as_path())?;
    write_all_json(out_dir.as_path(), &all_json)?;
    write_tags_json(out_dir.as_path(), &tags_json)?;
    Ok(())
}
