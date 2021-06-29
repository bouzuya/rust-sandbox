use crate::{post::list_posts, query::Query};
use std::{convert::TryFrom, path::PathBuf};

pub fn list(data_dir: PathBuf, json: bool, query: String) -> anyhow::Result<()> {
    let query = Query::try_from(query.as_str()).unwrap();
    let mut posts = list_posts(data_dir.as_path(), &query).unwrap();
    posts.sort();
    posts.reverse();
    let mut output = vec![];
    for post in posts {
        if json {
            output.push(format!(
                r#"{{"date":"{}","title":"{}"}}"#,
                post.date, post.title
            ));
        } else {
            output.push(format!("{} {}", post.date, post.title));
        }
    }
    let output = if json {
        let mut s = String::new();
        s.push('[');
        s.push_str(&output.join(","));
        s.push(']');
        s
    } else {
        output.join("\n")
    };
    println!("{}", output);
    Ok(())
}
