use std::convert::TryFrom;

use crate::{
    bbn_repository::BbnRepository, entry_id::EntryId, hatena_blog::HatenaBlogRepository,
    timestamp::Timestamp,
};
use anyhow::Context;
use date_range::date::Date;
use hatena_blog::{Client, CreateEntryResponse, Entry, EntryParams, UpdateEntryResponse};
use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum UploadEntryError {
    #[error("no entry id")]
    NoEntryId,
    #[error("no entry")]
    NoEntry,
}

pub async fn upload_entry(
    date: Date,
    draft: bool,
    hatena_id: &str,
    bbn_repository: &BbnRepository,
    hatena_blog_repository: &HatenaBlogRepository,
    hatena_blog_client: &Client,
) -> anyhow::Result<(bool, EntryId)> {
    let entry_id = bbn_repository
        .find_id_by_date(date)?
        .context(UploadEntryError::NoEntryId)?;
    let (entry_meta, entry_content) = bbn_repository
        .find_entry_by_id(&entry_id)?
        .context(UploadEntryError::NoEntry)?;
    let updated = entry_meta.pubdate;
    let params = EntryParams::new(
        hatena_id.to_string(),
        entry_meta.title.clone(),
        entry_content,
        updated.to_rfc3339(),
        vec![],
        draft,
    );
    let res = match hatena_blog_repository
        .find_entry_by_updated(updated)
        .await?
    {
        None => {
            let response = hatena_blog_client.create_entry(params).await?;
            let body = response.to_string();
            let hatena_blog_entry = Entry::try_from(CreateEntryResponse::from(body.clone()))?;
            hatena_blog_repository
                .create_member_response(&hatena_blog_entry.id, Timestamp::now()?, body)
                .await?;
            (true, entry_id)
        }
        Some(entry) => {
            let response = hatena_blog_client.update_entry(&entry.id, params).await?;
            let body = response.to_string();
            let hatena_blog_entry = Entry::try_from(UpdateEntryResponse::from(body.clone()))?;
            hatena_blog_repository
                .create_member_response(&hatena_blog_entry.id, Timestamp::now()?, body)
                .await?;
            (false, entry_id)
        }
    };
    Ok(res)
}
