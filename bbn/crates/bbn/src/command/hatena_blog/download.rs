use crate::{
    bbn_repository::BbnRepository,
    config_repository::ConfigRepository,
    data::{DateTime, EntryId, EntryMeta, Timestamp},
    hatena_blog::{
        download_entry, HatenaBlogClient, HatenaBlogEntry, HatenaBlogRepository, IndexingId,
    },
};
use anyhow::Context;
use chrono::{Local, NaiveDateTime, TimeZone};
use date_range::date::Date;
use hatena_blog_api::{Entry, GetEntryResponse};
use std::{collections::BTreeSet, convert::TryFrom, str::FromStr, time::Duration};
use tokio::time::sleep;

async fn indexing(
    hatena_blog_repository: &HatenaBlogRepository,
    hatena_blog_client: &HatenaBlogClient,
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

    let mut next_page = None;
    loop {
        let response = hatena_blog_client
            .list_entries_in_page(next_page.as_deref())
            .await?;
        let collection_response_id = hatena_blog_repository
            .create_collection_response(Timestamp::now()?, response.clone())
            .await?;
        println!(
            "downloaded collection page: {}",
            next_page.unwrap_or_else(|| "(null)".to_string())
        );
        hatena_blog_repository
            .create_indexing_collection_response(indexing.id(), collection_response_id)
            .await?;
        match response.next_page(last_indexing_started_at)? {
            None => break,
            Some(page) => next_page = Some(page),
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
    let mut hatena_blog_entry_ids = BTreeSet::new();
    for response in hatena_blog_repository
        .find_collection_responses_by_indexing_id(indexing.id())
        .await?
    {
        for hatena_blog_entry_id in response.hatena_blog_entry_ids(last_indexing_started_at)? {
            hatena_blog_entry_ids.insert(hatena_blog_entry_id.to_string());
        }
    }
    for hatena_blog_entry_id in hatena_blog_entry_ids {
        hatena_blog_repository
            .create_member_request(Timestamp::now()?, hatena_blog_entry_id)
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
    hatena_blog_entry: HatenaBlogEntry,
    bbn_repository: &BbnRepository,
) -> anyhow::Result<()> {
    let entry = match bbn_repository.find_entry_by_id(&entry_id)? {
        None => crate::data::Entry::new(
            entry_id,
            EntryMeta::new(
                15,
                hatena_blog_entry.updated,
                vec![],
                hatena_blog_entry.title,
            ),
            hatena_blog_entry.content,
        ),
        Some(bbn_entry) => {
            let meta = bbn_entry.meta().clone();
            bbn_entry.update(
                hatena_blog_entry.content,
                EntryMeta::new(
                    meta.minutes,
                    hatena_blog_entry.updated,
                    meta.tags,
                    hatena_blog_entry.title,
                ),
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
    hatena_blog_client: &HatenaBlogClient,
) -> anyhow::Result<()> {
    if let Some(d) = date {
        let entry_id = download_entry(
            d,
            bbn_repository,
            hatena_blog_repository,
            hatena_blog_client,
        )
        .await?;
        println!("downloaded member id: {}", entry_id);

        parse_entry(hatena_blog_repository).await?;

        return if data_file_only {
            Ok(())
        } else {
            update_bbn_entries(Some(entry_id), bbn_repository, hatena_blog_repository).await
        };
    }

    let _indexing_id = indexing(hatena_blog_repository, hatena_blog_client).await?;

    for member_request in hatena_blog_repository
        .find_incomplete_member_requests()
        .await?
    {
        match hatena_blog_client
            .get_entry(&member_request.hatena_blog_entry_id)
            .await?
        {
            None => {
                // get_entry returns None. ... The entry has been deleted.
                hatena_blog_repository
                    .create_member_request_result(member_request.id, Timestamp::now()?, None)
                    .await?;
            }
            Some(response) => {
                let body = response.to_string();
                match hatena_blog_repository
                    .create_member_response(Timestamp::now()?, body)
                    .await
                {
                    Ok(member_response_id) => {
                        hatena_blog_repository
                            .create_member_request_result(
                                member_request.id,
                                Timestamp::now()?,
                                Some(member_response_id),
                            )
                            .await?;
                    }
                    Err(err) => {
                        hatena_blog_repository
                            .create_member_request_result(
                                member_request.id,
                                Timestamp::now()?,
                                None,
                            )
                            .await?;
                        return Err(err);
                    }
                }
                println!(
                    "downloaded member id: {}",
                    member_request.hatena_blog_entry_id
                );
                sleep(Duration::from_secs(1)).await;
            }
        }
    }

    parse_entry(hatena_blog_repository).await?;

    if data_file_only {
        Ok(())
    } else {
        update_bbn_entries(None, bbn_repository, hatena_blog_repository).await
    }
}

pub async fn download(data_file_only: bool, date: Option<Date>) -> anyhow::Result<()> {
    let config_repository = ConfigRepository::new();
    let config = config_repository
        .load()
        .context("The configuration file does not found. Use `bbn config` command.")?;
    let data_file = config.hatena_blog_data_file().to_path_buf();
    let data_dir = config.data_dir().to_path_buf();
    let credentials = config_repository.load_credentials().with_context(|| {
        format!(
            "The credential file does not found. {:?}",
            config_repository.credential_file_path()
        )
    })?;

    let bbn_repository = BbnRepository::new(data_dir);
    let hatena_blog_repository = HatenaBlogRepository::new(data_file).await?;
    let hatena_blog_client = HatenaBlogClient::new(
        credentials.hatena_id().to_string(),
        credentials.hatena_blog_id().to_string(),
        credentials.hatena_api_key().to_string(),
    );
    download_impl(
        data_file_only,
        date,
        &bbn_repository,
        &hatena_blog_repository,
        &hatena_blog_client,
    )
    .await
}
