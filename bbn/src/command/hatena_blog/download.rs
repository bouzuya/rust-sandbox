use crate::{
    bbn_repository::BbnRepository,
    config_repository::ConfigRepository,
    hatena_blog::{download_entry, HatenaBlogRepository},
    timestamp::Timestamp,
};
use anyhow::Context;
use date_range::date::Date;
use hatena_blog::{Client, Config, Entry};
use std::{collections::BTreeSet, convert::TryInto, time::Duration};
use tokio::time::sleep;

pub async fn download_from_hatena_blog(
    date: Option<Date>,
    hatena_api_key: String,
    hatena_blog_id: String,
    hatena_id: String,
) -> anyhow::Result<()> {
    let config_repository = ConfigRepository::new();
    let config = config_repository
        .load()
        .context("The configuration file does not found. Use `bbn config` command.")?;
    let data_file = config.hatena_blog_data_file().to_path_buf();
    let data_dir = config.data_dir().to_path_buf();

    let config = Config::new(&hatena_id, None, &hatena_blog_id, &hatena_api_key);
    let client = Client::new(&config);
    let hatena_blog_repository = HatenaBlogRepository::new(data_file).await?;

    if let Some(d) = date {
        let bbn_repository = BbnRepository::new(data_dir);
        let entry_id = download_entry(d, bbn_repository, hatena_blog_repository, client).await?;
        println!("downloaded member id: {}", entry_id);
        return Ok(());
    }

    let last_download_at = hatena_blog_repository.get_last_list_request_at().await?;
    let curr_download_at = Timestamp::now()?;
    println!(
        "last download date: {}",
        last_download_at
            .map(|at| at.to_rfc3339())
            .unwrap_or_default()
    );

    let mut entry_ids = BTreeSet::new();
    let mut next_page = None;
    loop {
        // send request
        let request_id = hatena_blog_repository
            .create_list_request(Timestamp::now()?, &next_page)
            .await?;
        let response = client.list_entries_in_page(next_page.as_deref()).await?;
        let body = response.to_string();
        hatena_blog_repository
            .create_list_response(request_id, body)
            .await?;

        // parse response
        let (next, entries): (Option<String>, Vec<Entry>) = response.try_into()?;
        let filtered = entries
            .iter()
            .take_while(|entry| match last_download_at {
                None => true,
                Some(last) => Timestamp::from_rfc3339(&entry.published)
                    .map(|published| last <= published)
                    .unwrap_or(false),
            })
            .collect::<Vec<&Entry>>();
        for entry in filtered.iter() {
            if entry_ids.insert(entry.id.to_string()) {
                println!("{} (published: {})", entry.id, entry.published);
                let updated = Timestamp::from_rfc3339(&entry.updated)?;
                let published = Timestamp::from_rfc3339(&entry.published)?;
                let edited = Timestamp::from_rfc3339(&entry.edited)?;
                hatena_blog_repository
                    .add(&entry.id, updated, published, edited)
                    .await?;
            }
        }

        // next
        match (next, filtered.len() == entries.len()) {
            (None, _) | (Some(_), false) => {
                break;
            }
            (Some(page), true) => {
                next_page = Some(page);
            }
        }
        sleep(Duration::from_secs(1)).await;
    }

    hatena_blog_repository
        .set_last_list_request_at(curr_download_at)
        .await?;
    println!(
        "updated last download date: {}",
        curr_download_at.to_rfc3339()
    );

    for entry_id in hatena_blog_repository.find_incomplete_entry_ids().await? {
        let response = client.get_entry(&entry_id).await?;
        let body = response.to_string();
        hatena_blog_repository
            .create_member_response(&entry_id, Timestamp::now()?, body)
            .await?;
        println!("downloaded member id: {}", entry_id);
        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
