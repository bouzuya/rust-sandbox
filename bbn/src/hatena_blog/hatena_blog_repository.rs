use crate::{
    data::Timestamp,
    hatena_blog::{
        HatenaBlogEntry, HatenaBlogEntryId, Indexing, IndexingId, MemberRequest, MemberRequestId,
        MemberResponseId,
    },
};
use anyhow::Context as _;
use hatena_blog::{Entry, EntryId};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteRow},
    Pool, Row, Sqlite,
};
use std::{path::PathBuf, str::FromStr};

#[derive(Debug)]
pub struct HatenaBlogRepository {
    data_file: PathBuf,
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

        Ok(Self { data_file, pool })
    }

    pub async fn create_collection_response(
        &self,
        at: Timestamp,
        body: String,
    ) -> anyhow::Result<i64> {
        Ok(
            sqlx::query(include_str!("../../sql/create_collection_response.sql"))
                .bind(i64::from(at))
                .bind(body)
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
            .bind(if entry.draft { 1_i64 } else { 0_i64 })
            .bind(i64::from(Timestamp::from_rfc3339(&entry.edited).unwrap()))
            .bind(i64::from(
                Timestamp::from_rfc3339(&entry.published).unwrap(),
            ))
            .bind(entry.title)
            .bind(i64::from(Timestamp::from_rfc3339(&entry.updated).unwrap()))
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
    ) -> anyhow::Result<Vec<String>> {
        let rows: Vec<(String,)> = sqlx::query_as(include_str!(
            "../../sql/find_collection_responses_by_indexing_id.sql"
        ))
        .bind(i64::from(indexing_id))
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|(body,)| body)
            .collect::<Vec<String>>())
    }

    pub async fn find_entries_updated_and_title(&self) -> anyhow::Result<Vec<(Timestamp, String)>> {
        let rows: Vec<(i64, String)> =
            sqlx::query_as(include_str!("../../sql/find_entries_updated_and_title.sql"))
                .fetch_all(&self.pool)
                .await?;
        Ok(rows
            .into_iter()
            .map(|(updated, title)| (Timestamp::from(updated), title))
            .collect::<Vec<(Timestamp, String)>>())
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

    pub async fn find_entry_by_updated(
        &self,
        updated: Timestamp,
    ) -> anyhow::Result<Option<HatenaBlogEntry>> {
        Ok(
            sqlx::query(include_str!("../../sql/find_entry_by_updated.sql"))
                .bind(i64::from(updated))
                .map(|row: SqliteRow| {
                    HatenaBlogEntry::from(Entry::new(
                        EntryId::from_str(row.get(0)).unwrap(),
                        row.get(6),
                        row.get(1),
                        vec![],
                        row.get(2),
                        Timestamp::from(row.get::<'_, i64, _>(7)).to_rfc3339(),
                        Timestamp::from(row.get::<'_, i64, _>(5)).to_rfc3339(),
                        Timestamp::from(row.get::<'_, i64, _>(4)).to_rfc3339(),
                        row.get::<'_, i64, _>(3) == 1_i64,
                    ))
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
                    at: Timestamp::from(at),
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
        Ok(row.map(|(at,)| Timestamp::from(at)))
    }

    pub async fn find_indexing(&self, id: IndexingId) -> anyhow::Result<Option<Indexing>> {
        let row: Option<(i64, i64)> = sqlx::query_as(include_str!("../../sql/find_indexing.sql"))
            .bind(i64::from(id))
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|(id, at)| {
            let id = IndexingId::from(id);
            let at = Timestamp::from(at);
            Indexing::new(id, at)
        }))
    }

    pub async fn find_last_parsed_at(&self) -> anyhow::Result<Option<Timestamp>> {
        let row: Option<(i64,)> = sqlx::query_as(include_str!("../../sql/find_last_parsed_at.sql"))
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|(at,)| Timestamp::from(at)))
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
