use crate::query::Query;
use serde_json::Value;
use std::{ffi::OsStr, fs, io, path::Path};
use thiserror::Error;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Post {
    pub date: String,
    pub title: String,
}

#[derive(Debug, Error)]
pub enum ListPostsError {
    #[error("read dir error")]
    ReadDir(#[from] io::Error),
}

fn get_date<'a>(path: &'a Path) -> Option<&'a str> {
    path.file_stem()
        .and_then(|os_str| os_str.to_str())
        .and_then(|s| s.get(0..10))
}

pub fn list_posts(path: &Path, query: &Query) -> Result<Vec<Post>, ListPostsError> {
    // data/YYYY/MM/YYYY-MM-DD(-TITLE).json
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
        match query.month() {
            None => months.push(path_buf),
            Some(mm) => {
                if path_buf.file_name() == Some(OsStr::new(mm)) {
                    months.push(path_buf);
                }
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
        match query.day() {
            None => days.push(path_buf),
            Some(dd) => {
                // YYYY-MM-DD(-TITLE).json
                if let Some(file_stem) = path_buf.file_stem() {
                    if let Some(file_stem) = file_stem.to_str() {
                        if file_stem.get(8..8 + 2) == Some(dd) {
                            days.push(path_buf);
                        }
                    }
                }
            }
        }
    }

    let mut posts = vec![];
    for day in days {
        let path_buf = path.join(day);
        let date = get_date(path_buf.as_path()).unwrap().to_string();
        let content = fs::read_to_string(path_buf)?;
        let json: Value = serde_json::from_str(&content).unwrap();
        let title = json.get("title").unwrap().as_str().unwrap().to_string();
        posts.push(Post { date, title });
    }
    Ok(posts)
}
