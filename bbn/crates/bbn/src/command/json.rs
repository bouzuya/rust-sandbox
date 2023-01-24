use anyhow::Context;
use bbn_repository::{BbnRepository, Query};
use pulldown_cmark::{html, Parser};
use regex::Regex;
use std::{
    collections::{BTreeMap, HashSet},
    convert::TryFrom,
    fs::{self, File},
    io::BufWriter,
    path::{Path, PathBuf},
};

use crate::config_repository::ConfigRepository;

// <https://github.com/bouzuya/kraken/tree/v4.0.2/doc#all-json>
// all json (`/posts.json`)
#[derive(serde::Serialize)]
pub struct AllJson(pub Vec<AllJsonItem>);

#[derive(serde::Serialize)]
pub struct AllJsonItem {
    pub date: String, // "YYYY-MM-DD"
    #[serde(skip_serializing)]
    pub id_title: Option<String>, // "title" (obsolete)
    pub minutes: u32,
    pub pubdate: String, // "YYYY-MM-DDTHH:MM:SSZ"
    pub tags: Vec<String>,
    pub title: String,
}

// <https://github.com/bouzuya/kraken/tree/v4.0.2/doc#daily-json>
#[derive(serde::Serialize)]
pub struct DailyJson {
    pub data: String, // "markdown"
    pub date: String, // "YYYY-MM-DD" in "+09:00"
    pub minutes: u32,
    pub html: String, // "<p>markdown</p>"
    #[serde(skip_serializing)]
    pub id_title: Option<String>, // "title" (obsolete)
    pub pubdate: String, // "YYYY-MM-DDTHH:MM:SSZ"
    pub tags: Vec<String>,
    pub title: String,
}

// <https://github.com/bouzuya/kraken/tree/v4.0.2/doc#tags-json>
// tags json (`/tags.json`)
#[derive(serde::Serialize)]
pub struct TagsJson(pub Vec<TagsJsonItem>);

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

fn write_daily_json(out_dir: &Path, daily_json: &DailyJson) -> anyhow::Result<()> {
    let date = daily_json.date.split('-').collect::<Vec<&str>>();
    let yyyy = date[0];
    let mm = date[1];
    let dd = date[2];
    let id_title = daily_json.id_title.as_deref().unwrap_or("diary");
    let file_names = vec![
        format!("{yyyy}/{mm}/{dd}.json"),
        format!("{yyyy}/{mm}/{dd}/index.json"),
        format!("{yyyy}/{mm}/{dd}/{id_title}.json"),
        format!("{yyyy}/{mm}/{dd}/{id_title}/index.json"),
    ];
    for file_name in file_names {
        let path = out_dir.join(file_name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, daily_json)?;
    }
    Ok(())
}

fn write_tags_json(out_dir: &Path, tags_json: &TagsJson) -> anyhow::Result<()> {
    let path = out_dir.join("tags.json");
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, tags_json)?;
    Ok(())
}

fn markdown_to_html(markdown: &str) -> String {
    let mut html_output = String::new();
    html::push_html(&mut html_output, Parser::new(markdown));
    html_output
}

fn parse_links(markdown: &str) -> HashSet<String> {
    let mut links = HashSet::new();
    let regex = Regex::new(r#"\[([0-9]{4}-[0-1][0-9]-[0-3][0-9])\]"#).unwrap();
    for captures in regex.captures_iter(markdown) {
        links.insert(captures.get(1).map(|m| m.as_str().to_owned()).unwrap());
    }
    links
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
        let content = bbn_repository
            .find_content_by_id(&entry_id)?
            .context("content not found")?;

        for name in meta.tags.clone() {
            *tag_count_map.entry(name).or_insert(0) += 1;
        }

        let all_json_item = AllJsonItem {
            date: entry_id.date().to_string(),
            id_title: entry_id.id_title().map(|s| s.to_owned()),
            minutes: u32::try_from(meta.minutes)?,
            pubdate: meta.pubdate.to_string(),
            tags: meta.tags.clone(),
            title: meta.title.clone(),
        };
        all_json_items.push(all_json_item);

        let html = markdown_to_html(&content);
        let daily_json = DailyJson {
            data: content,
            date: entry_id.date().to_string(),
            html,
            id_title: entry_id.id_title().map(|s| s.to_owned()),
            minutes: u32::try_from(meta.minutes)?,
            pubdate: meta.pubdate.to_string(),
            tags: meta.tags,
            title: meta.title,
        };
        write_daily_json(out_dir.as_path(), &daily_json)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            parse_links(
                "[2021-02-03] [2021-02-04]\n\n[2021-02-03]: https://blog.bouzuya.net/2021/02/03/"
            ),
            {
                let mut set = HashSet::new();
                set.insert("2021-02-03".to_owned());
                set.insert("2021-02-04".to_owned());
                set
            }
        );
    }
}
