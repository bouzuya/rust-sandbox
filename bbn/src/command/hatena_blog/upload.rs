use std::{collections::VecDeque, convert::TryFrom, str::FromStr};

use anyhow::{bail, Context};
use console::StyledObject;
use date_range::date::Date;
use hatena_blog::{Client, Config};

use crate::{
    bbn_repository::BbnRepository,
    config_repository::ConfigRepository,
    hatena_blog::{upload_entry, HatenaBlogRepository},
    query::Query,
};

pub async fn upload(
    date: Option<Date>,
    draft: bool,
    hatena_api_key: String,
    hatena_blog_id: String,
    hatena_id: String,
    interactive: bool,
) -> anyhow::Result<()> {
    let config_repository = ConfigRepository::new();
    let config = config_repository
        .load()
        .context("The configuration file does not found. Use `bbn config` command.")?;
    let data_dir = config.data_dir().to_path_buf();
    let bbn_repository = BbnRepository::new(data_dir);
    let hatena_blog_data_file = config.hatena_blog_data_file().to_path_buf();
    let hatena_blog_repository = HatenaBlogRepository::new(hatena_blog_data_file).await?;
    let config = Config::new(
        hatena_id.as_str(),
        None,
        hatena_blog_id.as_str(),
        hatena_api_key.as_str(),
    );
    let hatena_blog_client = Client::new(&config);
    if let Some(date) = date {
        let (created, entry_id) = upload_entry(
            date,
            draft,
            &hatena_id,
            &bbn_repository,
            &hatena_blog_repository,
            &hatena_blog_client,
        )
        .await?;
        println!(
            "{} {}",
            if created { "created" } else { "updated" },
            entry_id
        );
    } else {
        if !interactive {
            bail!("interfactive = false is not supported");
        }

        let query = Query::try_from("")?;
        let entry_ids = bbn_repository.find_ids_by_query(query)?;
        for entry_id in entry_ids {
            let bbn_entry = bbn_repository.find_entry_by_id(&entry_id)?.unwrap();
            let hatena_blog_entry = hatena_blog_repository
                .find_entry_by_updated(bbn_entry.meta().pubdate.into())
                .await?;
            let result = match hatena_blog_entry {
                None => None,
                Some(ref entry) => {
                    if bbn_entry.content() != entry.content {
                        Some(false)
                    } else {
                        Some(true)
                    }
                }
            };
            if result == Some(true) {
                continue;
            }
            println!(
                "{} {}",
                result.map(|b| if b { "eq" } else { "ne" }).unwrap_or("no"),
                entry_id
            );
            match hatena_blog_entry {
                None => println!("no entry"),
                Some(entry) => {
                    show_2line_diff(entry.content.as_str(), bbn_entry.content());
                }
            }

            let yes = dialoguer::Confirm::new()
                .with_prompt("upload ?")
                .interact()?;
            if yes {
                let date = Date::from_str(entry_id.to_string().get(0..10).unwrap())?;
                let (created, entry_id) = upload_entry(
                    date,
                    draft,
                    hatena_id.as_str(),
                    &bbn_repository,
                    &hatena_blog_repository,
                    &hatena_blog_client,
                )
                .await?;
                println!(
                    "{} {}",
                    if created { "created" } else { "updated" },
                    entry_id
                );
            }
        }
    }
    Ok(())
}

fn show_2line_diff(left: &str, right: &str) {
    let mut c = 0;
    let mut q = VecDeque::<StyledObject<String>>::new();
    for diff_result in diff::lines(left, right) {
        let (d, output) = match diff_result {
            diff::Result::Left(l) => (true, console::style(format!("-{}", l)).red()),
            diff::Result::Right(r) => (true, console::style(format!("+{}", r)).green()),
            diff::Result::Both(l, _) => (false, console::style(format!(" {}", l))),
        };
        q.push_back(output);
        if d {
            while let Some(o) = q.pop_front() {
                println!("{}", o);
            }
            c = 2;
        } else if c > 0 {
            c -= 1;
            while let Some(o) = q.pop_front() {
                println!("{}", o);
            }
        } else if q.len() >= 2 {
            q.pop_front();
        }
    }
}
