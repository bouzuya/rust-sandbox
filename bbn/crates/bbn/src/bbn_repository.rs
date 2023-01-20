use std::{
    convert::TryFrom,
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::Context;
use date_range::date::{Date, YearMonth};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{hatena_blog::HatenaBlogEntryId, query::Query};
use bbn_data::{DateTime, Entry, EntryId, EntryMeta};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct MetaJson {
    #[serde(skip_serializing_if = "Option::is_none")]
    hatena_blog_entry_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hatena_blog_entry_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hatena_blog_ignore: Option<bool>,
    minutes: u64,
    pubdate: String,
    tags: Vec<String>,
    title: String,
}

impl std::convert::TryFrom<MetaJson> for EntryMeta {
    type Error = anyhow::Error;

    fn try_from(json: MetaJson) -> Result<Self, Self::Error> {
        Ok(Self {
            hatena_blog_entry_id: json
                .hatena_blog_entry_id
                .map(|s| HatenaBlogEntryId::from_str(s.as_str()))
                .transpose()?
                .map(|id| id.to_string()),
            hatena_blog_entry_url: json.hatena_blog_entry_url,
            hatena_blog_ignore: json.hatena_blog_ignore,
            minutes: json.minutes,
            pubdate: DateTime::from_str(json.pubdate.as_str())?,
            tags: json.tags,
            title: json.title,
        })
    }
}

impl From<EntryMeta> for MetaJson {
    fn from(meta: EntryMeta) -> Self {
        Self {
            hatena_blog_entry_id: meta.hatena_blog_entry_id.map(|i| i.to_string()),
            hatena_blog_entry_url: meta.hatena_blog_entry_url,
            hatena_blog_ignore: meta.hatena_blog_ignore,
            minutes: meta.minutes,
            pubdate: meta.pubdate.to_string(),
            tags: meta.tags,
            title: meta.title,
        }
    }
}

#[derive(Debug)]
pub struct BbnRepository {
    data_dir: PathBuf,
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

    pub fn find_entry_by_id(&self, entry_id: &EntryId) -> anyhow::Result<Option<Entry>> {
        let meta = self.find_meta_by_id(entry_id)?;
        let content = self.find_content_by_id(entry_id)?;
        Ok(meta
            .zip(content)
            .map(|(meta, content)| Entry::new(entry_id.clone(), meta, content)))
    }

    pub fn find_id_by_date(&self, date: Date) -> anyhow::Result<Option<EntryId>> {
        let entry_ids = self.find_ids_by_year_month(date.year_month())?;
        Ok(entry_ids.into_iter().find(|id| id.date() == &date))
    }

    pub fn find_ids_by_query(&self, query: Query) -> anyhow::Result<Vec<EntryId>> {
        let posts = list_posts(self.data_dir.as_path(), &query)?;
        posts
            .into_iter()
            .map(|post| self.post_to_entry_id(post))
            .collect::<anyhow::Result<Vec<EntryId>>>()
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
        Ok(Some(EntryMeta::try_from(meta_json)?))
    }

    pub fn save(&self, entry: Entry) -> anyhow::Result<()> {
        // meta
        let meta_path = self
            .data_dir
            .join(entry.id().date().year().to_string())
            .join(entry.id().date().month().to_string())
            .join(format!("{}.json", entry.id()));
        fs::create_dir_all(meta_path.parent().context("no parent dir")?)?;
        let meta_json = MetaJson::from(entry.meta().clone());
        let meta_json_content = serde_json::to_string(&meta_json)?;
        fs::write(meta_path.as_path(), meta_json_content)?;
        // content
        let content_path = self
            .data_dir
            .join(entry.id().date().year().to_string())
            .join(entry.id().date().month().to_string())
            .join(format!("{}.md", entry.id()));
        fs::write(content_path.as_path(), entry.content())?;
        Ok(())
    }

    fn find_ids_by_year_month(&self, year_month: YearMonth) -> anyhow::Result<Vec<EntryId>> {
        let entry_dir = self
            .data_dir
            .join(year_month.year().to_string())
            .join(year_month.month().to_string());
        if !entry_dir.is_dir() {
            return Ok(vec![]);
        }
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

    fn post_to_entry_id(&self, post: Post) -> anyhow::Result<EntryId> {
        let date = Date::from_str(post.date.as_str())?;
        Ok(EntryId::new(date, post.id_title))
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Post {
    pub date: String,
    pub id_title: Option<String>,
}

#[derive(Debug, Error)]
enum ListPostsError {
    #[error("read dir error")]
    ReadDir(#[from] io::Error),
}

fn get_date(path: &Path) -> Option<&'_ str> {
    path.file_stem()
        .and_then(|os_str| os_str.to_str())
        .and_then(|s| s.get(0..10))
}

fn get_id_title(path: &Path) -> Option<&'_ str> {
    path.file_stem()
        .and_then(|os_str| os_str.to_str())
        .and_then(|s| s.get(11..))
}

fn list_posts(path: &Path, query: &Query) -> Result<Vec<Post>, ListPostsError> {
    // data/YYYY/MM/YYYY-MM-DD(-ID_TITLE).json
    list_posts_year(path, query)
}

fn list_posts_year(path: &Path, query: &Query) -> Result<Vec<Post>, ListPostsError> {
    let mut years = vec![];
    for dir_entry in path.read_dir()? {
        let path_buf = dir_entry?.path();
        if let Some(year) = path_buf.file_name() {
            if query.match_year(year) {
                years.push(path_buf);
            }
        }
    }
    let mut posts = vec![];
    for year in years {
        let path_buf = path.join(year);
        let dir_posts = list_posts_month(path_buf.as_path(), query)?;
        posts.extend(dir_posts);
    }
    posts.sort();
    Ok(posts)
}

fn list_posts_month(path: &Path, query: &Query) -> Result<Vec<Post>, ListPostsError> {
    let mut months = vec![];
    for dir_entry in path.read_dir()? {
        let path_buf = dir_entry?.path();
        if let Some(month) = path_buf.file_name() {
            if query.match_month(month) {
                months.push(path_buf);
            }
        }
    }
    let mut posts = vec![];
    for month in months {
        let path_buf = path.join(month);
        let dir_posts = list_posts_day(path_buf.as_path(), query)?;
        posts.extend(dir_posts);
    }
    Ok(posts)
}

fn list_posts_day(path: &Path, query: &Query) -> Result<Vec<Post>, ListPostsError> {
    let mut days = vec![];
    for dir_entry in path.read_dir()? {
        let path_buf = dir_entry?.path();
        if path_buf.extension() != Some(OsStr::new("json")) {
            continue;
        }
        // YYYY-MM-DD(-ID_TITLE).json
        if let Some(day) = path_buf
            .file_stem()
            .and_then(|s| s.to_str())
            .and_then(|s| s.get(8..8 + 2))
            .map(OsStr::new)
        {
            if query.match_day(day) {
                if let Some(date) = path_buf
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .and_then(|s| s.get(0..10))
                {
                    if query.match_date(date) {
                        days.push(path_buf);
                    }
                }
            }
        }
    }

    let mut posts = vec![];
    for day in days {
        let path_buf = path.join(day);
        let date = get_date(path_buf.as_path()).unwrap().to_string();
        let id_title = get_id_title(path_buf.as_path()).map(|s| s.to_string());
        posts.push(Post { date, id_title });
    }
    Ok(posts)
}

#[cfg(test)]
mod tests {
    use std::{convert::TryFrom, fs};

    use tempfile::tempdir;

    use bbn_data::{DateTime, EntryId};

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
    fn find_entry_by_id_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let data_dir = create_test_dir(temp_dir.path())?;
        let repository = BbnRepository::new(data_dir);
        assert_eq!(
            repository.find_entry_by_id(&EntryId::from_str("2021-07-06")?)?,
            Some(Entry::new(
                EntryId::from_str("2021-07-06")?,
                EntryMeta::new(
                    5,
                    DateTime::from_str("2021-07-06T23:59:59+09:00")?,
                    vec!["tag1".to_string()],
                    "TITLE1".to_string()
                ),
                "CONTENT1".to_string()
            )),
        );
        assert_eq!(
            repository.find_entry_by_id(&EntryId::from_str("2021-07-07-id1")?)?,
            Some(Entry::new(
                EntryId::from_str("2021-07-07-id1")?,
                EntryMeta::new(
                    6,
                    DateTime::from_str("2021-07-07T23:59:59+09:00")?,
                    vec![],
                    "TITLE2".to_string()
                ),
                "CONTENT2".to_string()
            )),
        );
        assert_eq!(
            repository.find_entry_by_id(&EntryId::from_str("2021-07-08")?)?,
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
        // entry_dir does not exist
        assert_eq!(
            repository.find_id_by_date(Date::from_str("1970-01-01")?)?,
            None
        );
        Ok(())
    }

    #[test]
    fn find_ids_by_query_test() -> anyhow::Result<()> {
        // TODO: Query

        let temp_dir = tempdir()?;
        let data_dir = create_test_dir(temp_dir.path())?;

        let repository = BbnRepository::new(data_dir);
        let entry_ids = repository.find_ids_by_query(Query::try_from("")?)?;

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
    fn find_meta_by_id_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let data_dir = create_test_dir(temp_dir.path())?;
        let repository = BbnRepository::new(data_dir);
        assert_eq!(
            repository.find_meta_by_id(&EntryId::from_str("2021-07-06")?)?,
            Some(EntryMeta::new(
                5,
                DateTime::from_str("2021-07-06T23:59:59+09:00")?,
                vec!["tag1".to_string()],
                "TITLE1".to_string()
            )),
        );
        assert_eq!(
            repository.find_meta_by_id(&EntryId::from_str("2021-07-07-id1")?)?,
            Some(EntryMeta::new(
                6,
                DateTime::from_str("2021-07-07T23:59:59+09:00")?,
                vec![],
                "TITLE2".to_string()
            )),
        );
        assert_eq!(
            repository.find_meta_by_id(&EntryId::from_str("2021-07-08")?)?,
            None
        );
        Ok(())
    }

    #[test]
    fn save_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let data_dir = temp_dir.path().join("data");
        let repository = BbnRepository::new(data_dir.clone());

        repository.save(Entry::new(
            EntryId::from_str("2021-07-06")?,
            EntryMeta::new(
                5,
                DateTime::from_str("2021-07-06T23:59:59+09:00")?,
                vec!["tag1".to_string()],
                "TITLE1".to_string(),
            ),
            "CONTENT1".to_string(),
        ))?;
        assert_eq!(
            fs::read_to_string(data_dir.join("2021").join("07").join("2021-07-06.json"))?,
            r#"{"minutes":5,"pubdate":"2021-07-06T23:59:59+09:00","tags":["tag1"],"title":"TITLE1"}"#
        );
        assert_eq!(
            fs::read_to_string(data_dir.join("2021").join("07").join("2021-07-06.md"))?,
            "CONTENT1"
        );

        repository.save(Entry::new(
            EntryId::from_str("2021-07-07-id1")?,
            EntryMeta::new(
                6,
                DateTime::from_str("2021-07-07T23:59:59+09:00")?,
                vec![],
                "TITLE2".to_string(),
            ),
            "CONTENT2".to_string(),
        ))?;
        assert_eq!(
            fs::read_to_string(data_dir.join("2021").join("07").join("2021-07-07-id1.json"))?,
            r#"{"minutes":6,"pubdate":"2021-07-07T23:59:59+09:00","tags":[],"title":"TITLE2"}"#
        );
        assert_eq!(
            fs::read_to_string(data_dir.join("2021").join("07").join("2021-07-07-id1.md"))?,
            "CONTENT2"
        );

        // update
        repository.save(Entry::new(
            EntryId::from_str("2021-07-06")?,
            EntryMeta::new(
                6,
                DateTime::from_str("2021-07-07T23:59:59+09:00")?,
                vec![],
                "TITLE2".to_string(),
            ),
            "CONTENT2".to_string(),
        ))?;
        assert_eq!(
            fs::read_to_string(data_dir.join("2021").join("07").join("2021-07-06.json"))?,
            r#"{"minutes":6,"pubdate":"2021-07-07T23:59:59+09:00","tags":[],"title":"TITLE2"}"#
        );
        assert_eq!(
            fs::read_to_string(data_dir.join("2021").join("07").join("2021-07-06.md"))?,
            "CONTENT2"
        );
        Ok(())
    }
}
