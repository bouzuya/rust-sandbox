use crate::post::list_posts;
use crate::query::Query;
use date_range::date::Date;
use std::convert::TryFrom;
use std::path::PathBuf;

pub fn view(data_dir: PathBuf, date: Date) -> anyhow::Result<()> {
    let query_string = format!("date:{}", date);
    let query = Query::try_from(query_string.as_str()).unwrap();
    let posts = list_posts(data_dir.as_path(), &query).unwrap();
    let post = posts.first().unwrap();
    println!(
        "{} {} https://blog.bouzuya.net/{}/",
        post.date,
        post.title,
        date.to_string().replace('-', "/")
    );
    Ok(())
}
