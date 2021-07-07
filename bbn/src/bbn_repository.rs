use std::{
    convert::TryFrom,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::Context;
use date_range::date::{Date, YearMonth};
use serde_json::Value;

use crate::{
    entry::Entry,
    entry_id::EntryId,
    entry_meta::EntryMeta,
    post::{list_posts, Post},
    query::Query,
    timestamp::Timestamp,
};

#[derive(Debug, serde::Deserialize)]
struct MetaJson {
    minutes: u64,
    pubdate: String,
    tags: Vec<String>,
    title: String,
}

impl MetaJson {
    fn into_meta(self) -> anyhow::Result<EntryMeta> {
        Ok(EntryMeta {
            minutes: self.minutes,
            pubdate: Timestamp::from_rfc3339(self.pubdate.as_str())?,
            tags: self.tags,
            title: self.title,
        })
    }
}

#[derive(Debug)]
pub struct BbnRepository {
    data_dir: PathBuf,
}

fn get_bbn_entry(data_dir: &Path, entry_id: EntryId) -> anyhow::Result<crate::entry::Entry> {
    let date = entry_id.date();
    let id_title = entry_id.id_title();
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
    Ok(crate::entry::Entry {
        content,
        entry_id,
        minutes,
        pubdate,
        title,
    })
}

impl BbnRepository {
    pub fn new(data_dir: PathBuf) -> Self {
        Self { data_dir }
    }

    pub fn find_content_by_id(&self, entry_id: &EntryId) -> anyhow::Result<Option<String>> {
        let path = self
            .data_dir
            .join(entry_id.date().year().to_string())
            .join(entry_id.date().month().to_string())
            .join(format!("{}.md", entry_id));
        if !path.is_file() {
            return Ok(None);
        }
        Ok(Some(fs::read_to_string(path)?))
    }

    pub fn find_id_by_date(&self, date: Date) -> anyhow::Result<Option<EntryId>> {
        let entry_ids = self.find_ids_by_year_month(date.year_month())?;
        Ok(entry_ids.into_iter().find(|id| id.date() == &date))
    }

    pub fn find_meta_by_id(&self, entry_id: &EntryId) -> anyhow::Result<Option<EntryMeta>> {
        let path = self
            .data_dir
            .join(entry_id.date().year().to_string())
            .join(entry_id.date().month().to_string())
            .join(format!("{}.json", entry_id));
        if !path.is_file() {
            return Ok(None);
        }
        let json_content = fs::read_to_string(path)?;
        let meta_json = serde_json::from_str::<'_, MetaJson>(json_content.as_str())?;
        Ok(Some(meta_json.into_meta()?))
    }

    pub fn find_entry_ids(&self) -> anyhow::Result<Vec<EntryId>> {
        let query = Query::try_from("")?;
        let posts = list_posts(self.data_dir.as_path(), &query)?;
        posts
            .into_iter()
            .map(|post| {
                let date: date_range::date::Date = post.date.as_str().parse()?;
                Ok(EntryId::new(date, post.id_title))
            })
            .collect::<anyhow::Result<Vec<EntryId>>>()
    }

    pub fn find_by_date(&self, date: Date) -> anyhow::Result<Option<Entry>> {
        let query_string = format!("date:{}", date);
        let query = Query::try_from(query_string.as_str()).unwrap();
        let posts = list_posts(self.data_dir.as_path(), &query).unwrap();
        match posts.first() {
            None => Ok(None),
            Some(post) => self.post_to_entry(post),
        }
    }

    fn find_ids_by_year_month(&self, year_month: YearMonth) -> anyhow::Result<Vec<EntryId>> {
        let entry_dir = self
            .data_dir
            .join(year_month.year().to_string())
            .join(year_month.month().to_string());
        let mut entry_ids = vec![];
        for dir_entry in entry_dir.read_dir()? {
            let dir_entry_path = dir_entry?.path();
            if !dir_entry_path.is_file() {
                continue;
            }
            if dir_entry_path.extension() != Some(OsStr::new("json")) {
                continue;
            }
            if let Some(file) = dir_entry_path.file_stem().and_then(|s| s.to_str()) {
                entry_ids.push(EntryId::from_str(file)?);
            }
        }
        Ok(entry_ids)
    }

    fn post_to_entry(&self, post: &Post) -> anyhow::Result<Option<Entry>> {
        let date = Date::from_str(post.date.as_str())?;
        let entry_id = EntryId::new(date, post.id_title.clone());
        get_bbn_entry(self.data_dir.as_path(), entry_id).map(Some)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use crate::entry_id::EntryId;

    use super::*;

    fn create_test_dir(temp_dir: &Path) -> anyhow::Result<PathBuf> {
        let data_dir = temp_dir.join("data");
        let entry_dir = data_dir.join("2021").join("07");
        fs::create_dir_all(entry_dir.as_path())?;
        let entry_meta_path = entry_dir.join("2021-07-06.json");
        let entry_content_path = entry_dir.join("2021-07-06.md");
        fs::write(
            entry_meta_path.as_path(),
            r#"{"minutes":5,"pubdate":"2021-07-06T23:59:59+09:00","tags":["tag1"],"title":"TITLE1"}"#,
        )?;
        fs::write(entry_content_path.as_path(), r#"CONTENT1"#)?;
        let entry_meta_path = entry_dir.join("2021-07-07-id1.json");
        let entry_content_path = entry_dir.join("2021-07-07-id1.md");
        fs::write(
            entry_meta_path.as_path(),
            r#"{"minutes":6,"pubdate":"2021-07-07T23:59:59+09:00","tags":[],"title":"TITLE2"}"#,
        )?;
        fs::write(entry_content_path.as_path(), r#"CONTENT2"#)?;
        Ok(data_dir)
    }

    #[test]
    fn find_entry_ids_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let data_dir = create_test_dir(temp_dir.path())?;

        let repository = BbnRepository::new(data_dir);
        let entry_ids = repository.find_entry_ids()?;

        assert_eq!(
            entry_ids,
            vec![
                EntryId::from_str("2021-07-06")?,
                EntryId::from_str("2021-07-07-id1")?
            ]
        );
        Ok(())
    }

    #[test]
    fn find_by_date_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let data_dir = create_test_dir(temp_dir.path())?;
        let repository = BbnRepository::new(data_dir);
        assert_eq!(
            repository.find_by_date(Date::from_str("2021-07-06")?)?,
            Some(Entry {
                content: "CONTENT1".to_string(),
                entry_id: EntryId::new(Date::from_str("2021-07-06")?, None),
                minutes: 5,
                pubdate: Timestamp::from_rfc3339("2021-07-06T23:59:59+09:00")?,
                title: "TITLE1".to_string(),
            })
        );
        assert_eq!(
            repository.find_by_date(Date::from_str("2021-07-07")?)?,
            Some(Entry {
                content: "CONTENT2".to_string(),
                entry_id: EntryId::new(Date::from_str("2021-07-07")?, Some("id1".to_string())),
                minutes: 6,
                pubdate: Timestamp::from_rfc3339("2021-07-07T23:59:59+09:00")?,
                title: "TITLE2".to_string(),
            })
        );
        assert_eq!(
            repository.find_by_date(Date::from_str("2021-07-08")?)?,
            None
        );
        Ok(())
    }

    #[test]
    fn find_content_by_id_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let data_dir = create_test_dir(temp_dir.path())?;
        let repository = BbnRepository::new(data_dir);
        assert_eq!(
            repository.find_content_by_id(&EntryId::from_str("2021-07-06")?)?,
            Some("CONTENT1".to_string()),
        );
        assert_eq!(
            repository.find_content_by_id(&EntryId::from_str("2021-07-07-id1")?)?,
            Some("CONTENT2".to_string()),
        );
        assert_eq!(
            repository.find_content_by_id(&EntryId::from_str("2021-07-08")?)?,
            None
        );
        Ok(())
    }

    #[test]
    fn find_id_by_date_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let data_dir = create_test_dir(temp_dir.path())?;
        let repository = BbnRepository::new(data_dir);
        assert_eq!(
            repository.find_id_by_date(Date::from_str("2021-07-06")?)?,
            Some(EntryId::new(Date::from_str("2021-07-06")?, None)),
        );
        assert_eq!(
            repository.find_id_by_date(Date::from_str("2021-07-07")?)?,
            Some(EntryId::new(
                Date::from_str("2021-07-07")?,
                Some("id1".to_string())
            ))
        );
        assert_eq!(
            repository.find_id_by_date(Date::from_str("2021-07-08")?)?,
            None
        );
        Ok(())
    }

    #[test]
    fn find_meta_by_id_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let data_dir = create_test_dir(temp_dir.path())?;
        let repository = BbnRepository::new(data_dir);
        assert_eq!(
            repository.find_meta_by_id(&EntryId::from_str("2021-07-06")?)?,
            Some(EntryMeta {
                minutes: 5,
                pubdate: Timestamp::from_rfc3339("2021-07-06T23:59:59+09:00")?,
                tags: vec!["tag1".to_string()],
                title: "TITLE1".to_string()
            }),
        );
        assert_eq!(
            repository.find_meta_by_id(&EntryId::from_str("2021-07-07-id1")?)?,
            Some(EntryMeta {
                minutes: 6,
                pubdate: Timestamp::from_rfc3339("2021-07-07T23:59:59+09:00")?,
                tags: vec![],
                title: "TITLE2".to_string()
            }),
        );
        assert_eq!(
            repository.find_meta_by_id(&EntryId::from_str("2021-07-08")?)?,
            None
        );
        Ok(())
    }
}
