use crate::{bbn_hatena_blog::Repository, timestamp::Timestamp};
use hatena_blog::{Client, Config, Entry};
use std::{collections::BTreeSet, convert::TryInto, path::PathBuf, time::Duration};
use tokio::time::sleep;

pub async fn download_from_hatena_blog(
    data_file: PathBuf,
    hatena_api_key: String,
    hatena_blog_id: String,
    hatena_id: String,
) -> anyhow::Result<()> {
    let config = Config::new(&hatena_id, None, &hatena_blog_id, &hatena_api_key);
    let client = Client::new(&config);
    let repository = Repository::new(data_file).await?;

    let last_download_at = repository.get_last_list_request_at().await?;
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
        let request_id = repository
            .create_list_request(Timestamp::now()?, &next_page)
            .await?;
        let response = client.list_entries_in_page(next_page.as_deref()).await?;
        let body = response.to_string();
        repository.create_list_response(request_id, body).await?;

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
                repository
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

    repository
        .set_last_list_request_at(curr_download_at)
        .await?;
    println!(
        "updated last download date: {}",
        curr_download_at.to_rfc3339()
    );

    for entry_id in repository.find_incomplete_entry_ids().await? {
        let response = client.get_entry(&entry_id).await?;
        let body = response.to_string();
        repository
            .create_member_response(&entry_id, Timestamp::now()?, body)
            .await?;
        println!("downloaded member id: {}", entry_id);
        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
