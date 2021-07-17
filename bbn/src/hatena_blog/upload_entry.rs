use crate::{bbn_repository::BbnRepository, entry_id::EntryId, hatena_blog::HatenaBlogRepository};
use anyhow::Context;
use date_range::date::Date;
use hatena_blog::{Client, EntryParams};
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
    hatena_id: String,
    bbn_repository: BbnRepository,
    hatena_blog_repository: HatenaBlogRepository,
    hatena_blog_client: Client,
) -> anyhow::Result<(bool, EntryId)> {
    let entry_id = bbn_repository
        .find_id_by_date(date)?
        .context(UploadEntryError::NoEntryId)?;
    let (entry_meta, entry_content) = bbn_repository
        .find_entry_by_id(&entry_id)?
        .context(UploadEntryError::NoEntry)?;
    let updated = entry_meta.pubdate;
    let params = EntryParams::new(
        hatena_id,
        entry_meta.title.clone(),
        entry_content,
        updated.to_rfc3339(),
        vec![],
        draft,
    );
    match hatena_blog_repository
        .find_entry_by_updated(updated)
        .await?
    {
        None => {
            hatena_blog_client.create_entry(params).await?;
            Ok((true, entry_id))
        }
        Some(entry) => {
            hatena_blog_client.update_entry(&entry.id, params).await?;
            Ok((false, entry_id))
        }
    }
}
