use crate::{
    bbn_repository::BbnRepository, entry_id::EntryId, hatena_blog::HatenaBlogRepository,
    timestamp::Timestamp,
};
use anyhow::Context;
use date_range::date::Date;
use hatena_blog::Client;
use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum DownloadEntryError {
    #[error("no entry id")]
    NoEntryId,
    #[error("no entry meta")]
    NoEntryMeta,
    #[error("no hatena-blog entry")]
    NoHatenaBlogEntry,
}

pub async fn download_entry(
    date: Date,
    bbn_repository: BbnRepository,
    hatena_blog_repository: HatenaBlogRepository,
    hatena_blog_client: Client,
) -> anyhow::Result<EntryId> {
    let entry_id = bbn_repository
        .find_id_by_date(date)?
        .with_context(|| DownloadEntryError::NoEntryId)?;
    let entry_meta = bbn_repository
        .find_meta_by_id(&entry_id)?
        .with_context(|| DownloadEntryError::NoEntryMeta)?;
    let hatena_blog_entry = hatena_blog_repository
        .find_entry_by_updated(entry_meta.pubdate)
        .await?
        .with_context(|| DownloadEntryError::NoHatenaBlogEntry)?;
    let response = hatena_blog_client.get_entry(&hatena_blog_entry.id).await?;
    let body = response.to_string();
    hatena_blog_repository
        .create_member_response(&hatena_blog_entry.id, Timestamp::now()?, body)
        .await?;
    Ok(entry_id)
}
