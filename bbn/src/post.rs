use crate::query::Query;
use serde_json::Value;
use std::{ffi::OsStr, fs, io, path::Path};
use thiserror::Error;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Post {
    pub date: String,
    pub title: String,
    pub id_title: Option<String>,
}

#[derive(Debug, Error)]
pub enum ListPostsError {
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

pub fn list_posts(path: &Path, query: &Query) -> Result<Vec<Post>, ListPostsError> {
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
            .map(|s| OsStr::new(s))
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
        let content = fs::read_to_string(path_buf)?;
        let json: Value = serde_json::from_str(&content).unwrap();
        let title = json.get("title").unwrap().as_str().unwrap().to_string();
        posts.push(Post {
            date,
            title,
            id_title,
        });
    }
    Ok(posts)
}
