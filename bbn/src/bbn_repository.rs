use std::{
    convert::TryFrom,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::Context;
use date_range::date::Date;
use serde_json::Value;

use crate::{
    entry::Entry,
    entry_id::EntryId,
    post::{list_posts, Post},
    query::Query,
    timestamp::Timestamp,
};

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

    #[test]
    fn test() -> anyhow::Result<()> {
        let temp = tempdir()?;
        let data_dir = temp.path().join("data");
        let entry_dir = data_dir.join("2021").join("07");
        fs::create_dir_all(entry_dir.as_path())?;
        let entry_meta_path = entry_dir.join("2021-07-06.json");
        let entry_content_path = entry_dir.join("2021-07-06.md");
        fs::write(entry_meta_path.as_path(), r#"{"title":"TITLE1"}"#)?;
        fs::write(entry_content_path.as_path(), r#""#)?;

        let repository = BbnRepository::new(data_dir);
        let entry_ids = repository.find_entry_ids()?;

        assert_eq!(entry_ids, vec![EntryId::new("2021-07-06".parse()?, None)]);
        Ok(())
    }

    #[test]
    fn find_by_date() -> anyhow::Result<()> {
        let temp = tempdir()?;
        let data_dir = temp.path().join("data");
        let entry_dir = data_dir.join("2021").join("07");
        fs::create_dir_all(entry_dir.as_path())?;
        let entry_meta_path = entry_dir.join("2021-07-06.json");
        let entry_content_path = entry_dir.join("2021-07-06.md");
        fs::write(
            entry_meta_path.as_path(),
            r#"{"minutes":5,"pubdate":"2021-07-06T23:59:59+09:00","title":"TITLE1"}"#,
        )?;
        fs::write(entry_content_path.as_path(), r#"CONTENT1"#)?;
        let entry_meta_path = entry_dir.join("2021-07-07-id1.json");
        let entry_content_path = entry_dir.join("2021-07-07-id1.md");
        fs::write(
            entry_meta_path.as_path(),
            r#"{"minutes":6,"pubdate":"2021-07-07T23:59:59+09:00","title":"TITLE2"}"#,
        )?;
        fs::write(entry_content_path.as_path(), r#"CONTENT2"#)?;

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
}
