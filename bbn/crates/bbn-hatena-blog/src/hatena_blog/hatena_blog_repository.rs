use crate::hatena_blog::{
    HatenaBlogEntry, HatenaBlogEntryId, HatenaBlogListEntriesResponse, Indexing, IndexingId,
    MemberRequest, MemberRequestId, MemberResponseId,
};
use anyhow::Context as _;
use bbn_data::{DateTime, EntryMeta, Timestamp};
use hatena_blog_api::{Entry, EntryId, FixedDateTime};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteRow},
    Pool, Row, Sqlite,
};
use std::{path::PathBuf, str::FromStr};

#[derive(Debug)]
pub struct HatenaBlogRepository {
    pool: Pool<Sqlite>,
}

impl HatenaBlogRepository {
    pub async fn new(data_file: PathBuf) -> anyhow::Result<Self> {
        let options = SqliteConnectOptions::from_str(&format!(
            "sqlite:{}?mode=rwc",
            data_file.to_str().context("invalid path")?
        ))?
        .journal_mode(SqliteJournalMode::Delete);

        let pool = SqlitePoolOptions::new().connect_with(options).await?;
        let migrations = [
            include_str!("../../sql/create_table_entries.sql"),
            // response
            include_str!("../../sql/create_table_collection_responses.sql"),
            include_str!("../../sql/create_table_member_responses.sql"),
            // indexing
            include_str!("../../sql/create_table_indexings.sql"),
            include_str!("../../sql/create_table_indexing_collection_responses.sql"),
            include_str!("../../sql/create_table_successful_indexings.sql"),
            // member_requests
            include_str!("../../sql/create_table_member_requests.sql"),
            include_str!("../../sql/create_table_member_request_results.sql"),
        ];
        for migration in migrations.iter() {
            sqlx::query(migration).execute(&pool).await?;
        }

        Ok(Self { pool })
    }

    pub async fn create_collection_response(
        &self,
        at: Timestamp,
        response: HatenaBlogListEntriesResponse,
    ) -> anyhow::Result<i64> {
        Ok(
            sqlx::query(include_str!("../../sql/create_collection_response.sql"))
                .bind(i64::from(at))
                .bind(response.body())
                .execute(&self.pool)
                .await?
                .last_insert_rowid(),
        )
    }

    pub async fn create_entry(&self, entry: Entry, parsed_at: Timestamp) -> anyhow::Result<i64> {
        Ok(sqlx::query(include_str!("../../sql/create_entry.sql"))
            .bind(entry.id.to_string())
            .bind(entry.author_name)
            .bind(entry.content)
            .bind(i64::from(entry.draft))
            .bind(i64::from(Timestamp::from(DateTime::from(entry.edited))))
            .bind(entry.edit_url)
            .bind(i64::from(Timestamp::from(DateTime::from(entry.published))))
            .bind(entry.title)
            .bind(i64::from(Timestamp::from(DateTime::from(entry.updated))))
            .bind(entry.url)
            .bind(i64::from(parsed_at))
            .execute(&self.pool)
            .await?
            .last_insert_rowid())
    }

    pub async fn create_indexing(&self) -> anyhow::Result<Indexing> {
        let at = Timestamp::now()?;
        let id = IndexingId::from(
            sqlx::query(include_str!("../../sql/create_indexing.sql"))
                .bind(i64::from(at))
                .execute(&self.pool)
                .await?
                .last_insert_rowid(),
        );
        Ok(Indexing::new(id, at))
    }

    pub async fn create_indexing_collection_response(
        &self,
        indexing_id: IndexingId,
        collection_response_id: i64,
    ) -> anyhow::Result<i64> {
        Ok(sqlx::query(include_str!(
            "../../sql/create_indexing_collection_response.sql"
        ))
        .bind(i64::from(indexing_id))
        .bind(collection_response_id)
        .execute(&self.pool)
        .await?
        .last_insert_rowid())
    }

    pub async fn create_member_request(
        &self,
        at: Timestamp,
        entry_id: String,
    ) -> anyhow::Result<MemberRequestId> {
        Ok(MemberRequestId::from(
            sqlx::query(include_str!("../../sql/create_member_request.sql"))
                .bind(i64::from(at))
                .bind(entry_id)
                .execute(&self.pool)
                .await?
                .last_insert_rowid(),
        ))
    }

    pub async fn create_member_request_result(
        &self,
        member_request_id: MemberRequestId,
        at: Timestamp,
        member_response_id: Option<MemberResponseId>,
    ) -> anyhow::Result<()> {
        sqlx::query(include_str!("../../sql/create_member_request_result.sql"))
            .bind(i64::from(member_request_id))
            .bind(i64::from(at))
            .bind(member_response_id.map(i64::from))
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn create_member_response(
        &self,
        at: Timestamp,
        body: String,
    ) -> anyhow::Result<MemberResponseId> {
        Ok(MemberResponseId::from(
            sqlx::query(include_str!("../../sql/create_member_response.sql"))
                .bind(i64::from(at))
                .bind(body)
                .execute(&self.pool)
                .await?
                .last_insert_rowid(),
        ))
    }

    pub async fn create_successful_indexing(
        &self,
        indexing_id: IndexingId,
        at: Timestamp,
    ) -> anyhow::Result<i64> {
        Ok(
            sqlx::query(include_str!("../../sql/create_successful_indexing.sql"))
                .bind(i64::from(indexing_id))
                .bind(i64::from(at))
                .execute(&self.pool)
                .await?
                .last_insert_rowid(),
        )
    }

    pub async fn delete_entry(&self, entry_id: &EntryId) -> anyhow::Result<()> {
        sqlx::query(include_str!("../../sql/delete_entry.sql"))
            .bind(entry_id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn find_collection_responses_by_indexing_id(
        &self,
        indexing_id: IndexingId,
    ) -> anyhow::Result<Vec<HatenaBlogListEntriesResponse>> {
        let rows: Vec<(String,)> = sqlx::query_as(include_str!(
            "../../sql/find_collection_responses_by_indexing_id.sql"
        ))
        .bind(i64::from(indexing_id))
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|(body,)| HatenaBlogListEntriesResponse::from(body))
            .collect::<Vec<HatenaBlogListEntriesResponse>>())
    }

    pub async fn find_entries_updated_and_title(&self) -> anyhow::Result<Vec<(Timestamp, String)>> {
        let rows: Vec<(i64, String)> =
            sqlx::query_as(include_str!("../../sql/find_entries_updated_and_title.sql"))
                .fetch_all(&self.pool)
                .await?;
        rows.into_iter()
            .map(|(updated, title)| Ok((Timestamp::try_from(updated)?, title)))
            .collect::<anyhow::Result<Vec<(Timestamp, String)>>>()
    }

    pub async fn find_entries_waiting_for_parsing(
        &self,
        last_parsed_at: Option<Timestamp>,
    ) -> anyhow::Result<Vec<String>> {
        let rows: Vec<(String,)> = sqlx::query_as(include_str!(
            "../../sql/find_entries_waiting_for_parsing.sql"
        ))
        .bind(last_parsed_at.map(i64::from))
        .bind(last_parsed_at.map(i64::from))
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|(body,)| body)
            .collect::<Vec<String>>())
    }

    pub async fn find_entry_by_entry_meta(
        &self,
        entry_meta: &EntryMeta,
    ) -> anyhow::Result<Option<HatenaBlogEntry>> {
        match entry_meta.hatena_blog_entry_id.clone() {
            None => self.find_entry_by_updated(entry_meta.pubdate.into()).await,
            Some(entry_id) => {
                self.find_entry_by_id(HatenaBlogEntryId::from_str(entry_id.as_str())?)
                    .await
            }
        }
    }

    async fn find_entry_by_id(
        &self,
        hatena_blog_entry_id: HatenaBlogEntryId,
    ) -> anyhow::Result<Option<HatenaBlogEntry>> {
        let f = |i: i64| -> anyhow::Result<FixedDateTime> {
            Ok(FixedDateTime::from(DateTime::local_from_timestamp(
                Timestamp::try_from(i)?,
            )))
        };
        Ok(sqlx::query(include_str!("../../sql/find_entry_by_id.sql"))
            .bind(hatena_blog_entry_id.to_string())
            .map(|row: SqliteRow| {
                HatenaBlogEntry::from(Entry {
                    author_name: row.get("author_name"),
                    categories: vec![],
                    content: row.get("content"),
                    draft: row.get::<'_, i64, _>("draft") == 1_i64,
                    edit_url: row.get("edit_url"),
                    edited: f(row.get::<'_, i64, _>("edited")).unwrap(),
                    id: EntryId::from_str(row.get("entry_id")).unwrap(),
                    published: f(row.get::<'_, i64, _>("published")).unwrap(),
                    title: row.get("title"),
                    updated: f(row.get::<'_, i64, _>("updated")).unwrap(),
                    url: row.get("url"),
                })
            })
            .fetch_optional(&self.pool)
            .await?)
    }

    pub async fn find_entry_by_updated(
        &self,
        updated: Timestamp,
    ) -> anyhow::Result<Option<HatenaBlogEntry>> {
        let f = |i: i64| -> anyhow::Result<FixedDateTime> {
            Ok(FixedDateTime::from(DateTime::local_from_timestamp(
                Timestamp::try_from(i)?,
            )))
        };
        Ok(
            sqlx::query(include_str!("../../sql/find_entry_by_updated.sql"))
                .bind(i64::from(updated))
                .map(|row: SqliteRow| {
                    HatenaBlogEntry::from(Entry {
                        author_name: row.get("author_name"),
                        categories: vec![],
                        content: row.get("content"),
                        draft: row.get::<'_, i64, _>("draft") == 1_i64,
                        edit_url: row.get("edit_url"),
                        edited: f(row.get::<'_, i64, _>("edited")).unwrap(),
                        id: EntryId::from_str(row.get("entry_id")).unwrap(),
                        published: f(row.get::<'_, i64, _>("published")).unwrap(),
                        title: row.get("title"),
                        updated: f(row.get::<'_, i64, _>("updated")).unwrap(),
                        url: row.get("url"),
                    })
                })
                .fetch_optional(&self.pool)
                .await?,
        )
    }

    pub async fn find_incomplete_member_requests(&self) -> anyhow::Result<Vec<MemberRequest>> {
        let rows: Vec<(i64, i64, String)> = sqlx::query_as(include_str!(
            "../../sql/find_incomplete_member_requests.sql"
        ))
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|(id, at, entry_id)| -> anyhow::Result<MemberRequest> {
                Ok(MemberRequest {
                    id: MemberRequestId::from(id),
                    at: Timestamp::try_from(at)?,
                    hatena_blog_entry_id: HatenaBlogEntryId::from_str(entry_id.as_str())?,
                })
            })
            .collect::<anyhow::Result<Vec<MemberRequest>>>()
    }

    pub async fn find_last_successful_indexing_started_at(
        &self,
    ) -> anyhow::Result<Option<Timestamp>> {
        let row: Option<(i64,)> = sqlx::query_as(include_str!(
            "../../sql/find_last_successful_indexing_started_at.sql"
        ))
        .fetch_optional(&self.pool)
        .await?;
        row.map(|(at,)| Timestamp::try_from(at)).transpose()
    }

    #[allow(dead_code)]
    async fn find_indexing(&self, id: IndexingId) -> anyhow::Result<Option<Indexing>> {
        let row: Option<(i64, i64)> = sqlx::query_as(include_str!("../../sql/find_indexing.sql"))
            .bind(i64::from(id))
            .fetch_optional(&self.pool)
            .await?;
        row.map(|(id, at)| {
            let id = IndexingId::from(id);
            let at = Timestamp::try_from(at)?;
            Ok(Indexing::new(id, at))
        })
        .transpose()
    }

    pub async fn find_last_parsed_at(&self) -> anyhow::Result<Option<Timestamp>> {
        let row: Option<(i64,)> = sqlx::query_as(include_str!("../../sql/find_last_parsed_at.sql"))
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|(at,)| Timestamp::try_from(at)).transpose()?)
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn indexing_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let data_file = temp_dir.path().join("db");
        let repository = HatenaBlogRepository::new(data_file).await?;
        let created = repository.create_indexing().await?;
        let found = repository.find_indexing(created.id()).await?;
        assert_eq!(found, Some(created));
        Ok(())
    }
}
