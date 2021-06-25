use std::{fs, path::PathBuf, process::Command};

use anyhow::bail;

pub fn post_to_hatena_blog(
    data_dir: PathBuf,
    date: String,
    draft: bool,
    hatena_api_key: String,
    hatena_blog_id: String,
    hatena_id: String,
) -> anyhow::Result<()> {
    let split = date.split('-').collect::<Vec<&str>>();
    if !(split.len() == 3 && split[0].len() == 4 && split[1].len() == 2 && split[2].len() == 2) {
        bail!("invalid date");
    }
    let md = data_dir
        .as_path()
        .join(split[0])
        .join(split[1])
        .join(date.as_str())
        .with_extension("md");
    let json = md.with_extension("json");
    #[derive(Debug, serde::Deserialize)]
    struct MetaJson {
        title: String,
        pubdate: String,
    }
    let meta_string = fs::read_to_string(json)?;
    let meta: MetaJson = serde_json::from_str(meta_string.as_str())?;
    let status = Command::new("hatena-blog")
        .arg("create")
        .args(&["--title", meta.title.as_str()])
        .args(&["--updated", meta.pubdate.as_str()])
        .args(&(if draft { vec!["--draft"] } else { vec![] }))
        .arg(md.as_path().as_os_str())
        .env("HATENA_API_KEY", hatena_api_key)
        .env("HATENA_BLOG_ID", hatena_blog_id)
        .env("HATENA_ID", hatena_id)
        .status()?;
    if status.success() {
        println!("{} {}", date, meta.title);
    }
    Ok(())
}
