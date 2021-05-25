mod query;

use query::Query;
use serde_json::Value;
use std::{
    convert::TryFrom,
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};
use structopt::StructOpt;
use thiserror::Error;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(name = "list", about = "Lists the blog posts")]
    List {
        #[structopt(long = "data-dir", help = "the data dir")]
        data_dir: PathBuf,
        #[structopt(name = "query", help = "query")]
        query: String,
    },
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Post {
    date: String,
    title: String,
}

#[derive(Debug, Error)]
enum ListPostsError {
    #[error("read dir error")]
    ReadDir(#[from] io::Error),
}

fn list_posts(path: &Path, query: &str) -> Result<Vec<Post>, ListPostsError> {
    let mut entries = vec![];
    if path.is_dir() {
        for dir_entry in path.read_dir()? {
            let path_buf = path.join(dir_entry?.path());
            let dir_posts = list_posts(path_buf.as_path(), query)?;
            entries.extend(dir_posts);
        }
    } else {
        if path.extension() == Some(OsStr::new("json")) {
            let date = path.file_stem().unwrap().to_str().unwrap().to_string();
            // YYYY-MM-DD
            if &date[5..] == query {
                let content = fs::read_to_string(path)?;
                let json: Value = serde_json::from_str(&content).unwrap();
                let title = json.get("title").unwrap().as_str().unwrap().to_string();
                entries.push(Post { date, title });
            }
        }
    }
    Ok(entries)
}

fn main() {
    let opt = Opt::from_args();
    match opt.subcommand {
        Subcommand::List { data_dir, query } => {
            let query = Query::try_from(query.as_str()).unwrap();
            let q = query.to_string().as_bytes()[7..]
                .iter()
                .map(|&b| char::from_u32(b as u32).unwrap())
                .collect::<String>();
            let mut posts = list_posts(data_dir.as_path(), &q).unwrap();
            posts.sort();
            posts.reverse();
            for post in posts {
                println!("{} {}", post.date, post.title);
            }
        }
    }
}
