use crate::{
    bbn_repository::BbnRepository,
    config_repository::ConfigRepository,
    datetime::DateTime,
    entry_id::EntryId,
    entry_meta::EntryMeta,
    hatena_blog::{download_entry, HatenaBlogRepository, IndexingId},
    query::Query,
    timestamp::Timestamp,
};
use anyhow::{anyhow, Context};
use chrono::{Local, NaiveDateTime, TimeZone};
use date_range::date::Date;
use hatena_blog::{Client, Config, Entry, GetEntryResponse, ListEntriesResponse};
use std::{
    collections::BTreeSet,
    convert::{TryFrom, TryInto},
    str::FromStr,
    time::Duration,
};
use tokio::time::sleep;

async fn indexing(
    hatena_blog_repository: &HatenaBlogRepository,
    client: &Client,
) -> anyhow::Result<IndexingId> {
    let last_indexing_started_at = hatena_blog_repository
        .find_last_successful_indexing_started_at()
        .await?;
    println!(
        "last indexing started at: {}",
        last_indexing_started_at
            .map(|at| at.to_rfc3339())
            .unwrap_or_else(|| "(null)".to_string())
    );

    let indexing = hatena_blog_repository.create_indexing().await?;
    println!("indexing started at: {}", indexing.at().to_rfc3339());

    let mut entry_ids = BTreeSet::new();
    let mut next_page = None;
    loop {
        // send request
        let response = client.list_entries_in_page(next_page.as_deref()).await?;
        let body = response.to_string();
        let collection_response_id = hatena_blog_repository
            .create_collection_response(Timestamp::now()?, body)
            .await?;
        println!(
            "downloaded collection page: {}",
            next_page.unwrap_or_else(|| "(null)".to_string())
        );
        hatena_blog_repository
            .create_indexing_collection_response(indexing.id(), collection_response_id)
            .await?;

        // parse response
        let (next, entries): (Option<String>, Vec<Entry>) = response.try_into()?;
        let filtered = entries
            .iter()
            .take_while(|entry| match last_indexing_started_at {
                None => true,
                Some(last) => Timestamp::from_rfc3339(&entry.published)
                    .map(|published| last <= published)
                    .unwrap_or(false),
            })
            .collect::<Vec<&Entry>>();
        for entry in filtered.iter() {
            if entry_ids.insert(entry.id.to_string()) {
                println!(
                    "parsed hatena_blog entry_id: {} (published: {})",
                    entry.id, entry.published
                );
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

    let indexing_succeeded_at = Timestamp::now()?;
    hatena_blog_repository
        .create_successful_indexing(indexing.id(), indexing_succeeded_at)
        .await?;
    println!(
        "indexing succeeded at: {}",
        indexing_succeeded_at.to_rfc3339()
    );

    // TODO: remove
    let mut entry_ids = BTreeSet::new();
    for body in hatena_blog_repository
        .find_collection_responses_by_indexing_id(indexing.id())
        .await?
    {
        let response = ListEntriesResponse::from(body);
        let (_, entries): (Option<String>, Vec<Entry>) = response.try_into()?;
        let filtered = entries
            .iter()
            .take_while(|entry| match last_indexing_started_at {
                None => true,
                Some(last) => Timestamp::from_rfc3339(&entry.published)
                    .map(|published| last <= published)
                    .unwrap_or(false),
            })
            .collect::<Vec<&Entry>>();
        for entry in filtered.iter() {
            entry_ids.insert(entry.id.to_string());
        }
    }
    for entry_id in entry_ids {
        hatena_blog_repository
            .create_member_request(Timestamp::now()?, entry_id)
            .await?;
    }

    Ok(indexing.id())
}

async fn parse_entry(hatena_blog_repository: &HatenaBlogRepository) -> anyhow::Result<()> {
    let last_parsed_at = hatena_blog_repository.find_last_parsed_at().await?;
    for body in hatena_blog_repository
        .find_entries_waiting_for_parsing(last_parsed_at)
        .await?
    {
        let entry = Entry::try_from(GetEntryResponse::from(body))?;
        let entry_id = entry.id.clone();
        hatena_blog_repository.delete_entry(&entry_id).await?;
        hatena_blog_repository
            .create_entry(entry, Timestamp::now()?)
            .await?;
        eprintln!("parsed member id: {}", entry_id);
    }
    Ok(())
}

fn update_bbn_entry(
    entry_id: EntryId,
    hatena_blog_entry: Entry,
    bbn_repository: &BbnRepository,
) -> anyhow::Result<()> {
    let timestamp = Timestamp::from_rfc3339(hatena_blog_entry.updated.as_str())?;
    let utc_naive_date_time = NaiveDateTime::from_timestamp(i64::from(timestamp), 0);
    let fixed_datetime = Local.from_utc_datetime(&utc_naive_date_time);
    let datetime = DateTime::from_str(&fixed_datetime.to_rfc3339())?;
    let entry = match bbn_repository.find_entry_by_id(&entry_id)? {
        None => crate::entry::Entry::new(
            entry_id,
            EntryMeta {
                minutes: 15,
                pubdate: datetime,
                tags: vec![],
                title: hatena_blog_entry.title,
            },
            hatena_blog_entry.content,
        ),
        Some(bbn_entry) => {
            let meta = bbn_entry.meta().clone();
            bbn_entry.update(
                hatena_blog_entry.content,
                EntryMeta {
                    minutes: meta.minutes,
                    pubdate: datetime,
                    tags: meta.tags,
                    title: hatena_blog_entry.title,
                },
            )
        }
    };
    bbn_repository.save(entry)
}

async fn update_bbn_entries(
    target_entry_id: Option<EntryId>,
    bbn_repository: &BbnRepository,
    hatena_blog_repository: &HatenaBlogRepository,
) -> anyhow::Result<()> {
    for (updated, _) in hatena_blog_repository
        .find_entries_updated_and_title()
        .await?
    {
        let utc_naive_date_time = NaiveDateTime::from_timestamp(i64::from(updated), 0);
        let fixed_datetime = Local.from_utc_datetime(&utc_naive_date_time);
        let datetime = DateTime::from_str(&fixed_datetime.to_rfc3339())?;
        let date = Date::from_str(datetime.to_string().get(0..10).unwrap())?;
        println!("{:?} {:?}", updated, date);
        let entry_id = match bbn_repository.find_id_by_date(date)? {
            None => EntryId::new(date, None),
            Some(entry_id) => entry_id,
        };
        if let Some(ref target) = target_entry_id {
            if target != &entry_id {
                continue;
            }
        }
        let hatena_blog_entry = hatena_blog_repository
            .find_entry_by_updated(updated)
            .await?
            .unwrap();
        update_bbn_entry(entry_id, hatena_blog_entry, bbn_repository)?;
    }
    Ok(())
}

async fn download_impl(
    data_file_only: bool,
    date: Option<Date>,
    bbn_repository: &BbnRepository,
    hatena_blog_repository: &HatenaBlogRepository,
    hatena_blog_client: &Client,
) -> anyhow::Result<()> {
    if let Some(d) = date {
        let entry_id = download_entry(
            d,
            &bbn_repository,
            &hatena_blog_repository,
            &hatena_blog_client,
        )
        .await?;
        println!("downloaded member id: {}", entry_id);

        parse_entry(&hatena_blog_repository).await?;

        return if data_file_only {
            Ok(())
        } else {
            update_bbn_entries(Some(entry_id), &bbn_repository, &hatena_blog_repository).await
        };
    }

    let _indexing_id = indexing(&hatena_blog_repository, &hatena_blog_client).await?;

    for entry_id in hatena_blog_repository.find_incomplete_entry_ids().await? {
        let response = hatena_blog_client.get_entry(&entry_id).await?;
        let body = response.to_string();
        hatena_blog_repository
            .create_member_response(Timestamp::now()?, body)
            .await?;
        println!("downloaded member id: {}", entry_id);
        sleep(Duration::from_secs(1)).await;
    }

    parse_entry(&hatena_blog_repository).await?;

    if data_file_only {
        Ok(())
    } else {
        update_bbn_entries(None, &bbn_repository, &hatena_blog_repository).await
    }
}

pub async fn download(
    data_file_only: bool,
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

    let bbn_repository = BbnRepository::new(data_dir);
    let hatena_blog_repository = HatenaBlogRepository::new(data_file).await?;
    let hatena_blog_client_config = Config::new(&hatena_id, None, &hatena_blog_id, &hatena_api_key);
    let hatena_blog_client = Client::new(&hatena_blog_client_config);
    download_impl(
        data_file_only,
        date,
        &bbn_repository,
        &hatena_blog_repository,
        &hatena_blog_client,
    )
    .await
}
