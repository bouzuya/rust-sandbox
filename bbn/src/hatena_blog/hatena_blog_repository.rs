use crate::timestamp::Timestamp;
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
        sqlx::query(
            r#"
        CREATE TABLE IF NOT EXISTS entry_ids (
            entry_id TEXT PRIMARY KEY,
            updated INTEGER NOT NULL,
            published INTEGER NOT NULL,
            edited INTEGER NOT NULL
        )"#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
        CREATE TABLE IF NOT EXISTS entries (
            entry_id TEXT PRIMARY KEY,
            author_name TEXT NOT NULL,
            content TEXT NOT NULL,
            draft INTEGER NOT NULL,
            edited INTEGER NOT NULL,
            published INTEGER NOT NULL,
            title TEXT NOT NULL,
            updated INTEGER NOT NULL,
            parsed_at INTEGER NOT NULL,
            FOREIGN KEY (entry_id) REFERENCES entry_ids(entry_id)
        )"#,
        )
        .execute(&pool)
        .await?;

        // response

        sqlx::query(
            r#"
CREATE TABLE IF NOT EXISTS collection_responses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    at INTEGER NOT NULL,
    body TEXT NOT NULL
)
"#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
CREATE TABLE IF NOT EXISTS member_responses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    at TIMESTAMP NOT NULL,
    body TEXT NOT NULL
)
"#,
        )
        .execute(&pool)
        .await?;

        // indexing

        sqlx::query(
            r#"
CREATE TABLE IF NOT EXISTS indexings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    at INTEGER NOT NULL
)
"#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
CREATE TABLE IF NOT EXISTS indexing_collection_responses (
    indexing_id INTEGER NOT NULL,
    collection_response_id INTEGER NOT NULL,
    PRIMARY KEY (indexing_id, collection_response_id),
    FOREIGN KEY (indexing_id) REFERENCES indexings (id) ON DELETE CASCADE,
    FOREIGN KEY (collection_response_id) REFERENCES collection_responses (id) ON DELETE CASCADE
)
"#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
CREATE TABLE IF NOT EXISTS successful_indexings (
    indexing_id INTEGER PRIMARY KEY,
    at INTEGER NOT NULL,
    FOREIGN KEY (indexing_id) REFERENCES indexings (id) ON DELETE CASCADE
)
"#,
        )
        .execute(&pool)
        .await?;

        // member_requests

        sqlx::query(
            r#"
CREATE TABLE IF NOT EXISTS member_requests (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    at INTEGER NOT NULL,
    entry_id TEXT NOT NULL
)
"#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
CREATE TABLE IF NOT EXISTS member_request_results (
    member_request_id INTEGER PRIMARY KEY,
    at INTEGER NOT NULL,
    member_response_id INTEGER, -- nullable
    FOREIGN KEY (member_request_id) REFERENCES member_requests (id) ON DELETE CASCADE,
    FOREIGN KEY (member_response_id) REFERENCES member_responses (id)
)
"#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { data_file, pool })
    }

    pub async fn add(
        &self,
        entry_id: &EntryId,
        updated: Timestamp,
        published: Timestamp,
        edited: Timestamp,
    ) -> anyhow::Result<()> {
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM entry_ids WHERE entry_id = ?")
            .bind(entry_id.to_string())
            .fetch_one(&self.pool)
            .await?;
        if count > 0 {
            return Ok(());
        }
        Ok(sqlx::query(
            r#"
INSERT INTO entry_ids(entry_id, updated, published, edited)
VALUES (?, ?, ?, ?)
"#,
        )
        .bind(entry_id.to_string())
        .bind(i64::from(updated))
        .bind(i64::from(published))
        .bind(i64::from(edited))
        .execute(&self.pool)
        .await
        .map(|_| ())?)
    }

    pub async fn create_collection_response(
        &self,
        at: Timestamp,
        body: String,
    ) -> anyhow::Result<i64> {
        Ok(sqlx::query(
            r#"
INSERT INTO collection_responses(at, body)
VALUES (?, ?)
            "#,
        )
        .bind(i64::from(at))
        .bind(body)
        .execute(&self.pool)
        .await?
        .last_insert_rowid())
    }

    pub async fn create_entry(&self, entry: Entry, parsed_at: Timestamp) -> anyhow::Result<i64> {
        Ok(sqlx::query(
            r#"
INSERT INTO entries(
  entry_id,
  author_name,
  content,
  draft,
  edited,
  published,
  title,
  updated,
  parsed_at
)
VALUES (
  ?,
  ?,
  ?,
  ?,
  ?,
  ?,
  ?,
  ?,
  ?
)

"#,
        )
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

    pub async fn create_indexing(&self, at: Timestamp) -> anyhow::Result<i64> {
        Ok(sqlx::query(
            r#"
INSERT INTO indexings(at)
VALUES (?)
"#,
        )
        .bind(i64::from(at))
        .execute(&self.pool)
        .await?
        .last_insert_rowid())
    }

    pub async fn create_indexing_collection_response(
        &self,
        indexing_id: i64,
        collection_response_id: i64,
    ) -> anyhow::Result<i64> {
        Ok(sqlx::query(
            r#"
INSERT INTO indexing_collection_responses(
    indexing_id,
    collection_response_id
)
VALUES (?, ?)
"#,
        )
        .bind(indexing_id)
        .bind(collection_response_id)
        .execute(&self.pool)
        .await?
        .last_insert_rowid())
    }

    pub async fn create_member_request(
        &self,
        at: Timestamp,
        entry_id: String,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"
INSERT INTO member_requests(at, entry_id)
VALUES (?, ?)
            "#,
        )
        .bind(i64::from(at))
        .bind(entry_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn create_member_response(&self, at: Timestamp, body: String) -> anyhow::Result<()> {
        sqlx::query(
            r#"
INSERT INTO member_responses(at, body)
VALUES (?, ?)
            "#,
        )
        .bind(i64::from(at))
        .bind(body)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn create_successful_indexing(
        &self,
        indexing_id: i64,
        at: Timestamp,
    ) -> anyhow::Result<i64> {
        Ok(sqlx::query(
            r#"
INSERT INTO successful_indexings(indexing_id, at)
VALUES (?, ?)
            "#,
        )
        .bind(indexing_id)
        .bind(i64::from(at))
        .execute(&self.pool)
        .await?
        .last_insert_rowid())
    }

    pub async fn delete_entry(&self, entry_id: &EntryId) -> anyhow::Result<()> {
        sqlx::query(
            r#"
DELETE FROM entries
WHERE entries.entry_id = ?
            "#,
        )
        .bind(entry_id.to_string())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_collection_responses_by_indexing_id(
        &self,
        indexing_id: i64,
    ) -> anyhow::Result<Vec<String>> {
        let rows: Vec<(String,)> = sqlx::query_as(
            r#"
SELECT
    collection_responses.body
FROM indexing_collection_responses
INNER JOIN collection_responses
ON collection_responses.id = indexing_collection_responses.collection_response_id
WHERE
    indexing_id = ?
ORDER BY
    collection_responses.id ASC
"#,
        )
        .bind(indexing_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|(body,)| body)
            .collect::<Vec<String>>())
    }

    pub async fn find_entries_waiting_for_parsing(
        &self,
        last_parsed_at: Option<Timestamp>,
    ) -> anyhow::Result<Vec<String>> {
        let rows: Vec<(String,)> = sqlx::query_as(
            r#"
SELECT
    member_responses.body
FROM
    member_responses
WHERE ? IS NULL
OR member_responses.at > ?
ORDER BY member_responses.at ASC
                    "#,
        )
        .bind(last_parsed_at.map(i64::from))
        .bind(last_parsed_at.map(i64::from))
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|(body,)| body)
            .collect::<Vec<String>>())
    }

    pub async fn find_entry_by_updated(&self, updated: Timestamp) -> anyhow::Result<Option<Entry>> {
        Ok(sqlx::query(
            r#"
SELECT
  entry_id,
  author_name,
  content,
  draft,
  edited,
  published,
  title,
  updated
FROM entries
WHERE updated = ?
"#,
        )
        .bind(i64::from(updated))
        .map(|row: SqliteRow| {
            Entry::new(
                EntryId::from_str(row.get(0)).unwrap(),
                row.get(6),
                row.get(1),
                vec![],
                row.get(2),
                Timestamp::from(row.get::<'_, i64, _>(7)).to_rfc3339(),
                Timestamp::from(row.get::<'_, i64, _>(5)).to_rfc3339(),
                Timestamp::from(row.get::<'_, i64, _>(4)).to_rfc3339(),
                row.get::<'_, i64, _>(3) == 1_i64,
            )
        })
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn find_incomplete_entry_ids(&self) -> anyhow::Result<Vec<EntryId>> {
        let rows: Vec<(String,)> = sqlx::query_as(
            r#"
SELECT member_requests.entry_id
FROM member_requests
LEFT OUTER JOIN member_request_results
ON member_request_results.member_request_id = member_requests.id
WHERE member_request_results.member_request_id IS NULL
ORDER BY member_requests.at ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;
        rows.iter()
            .map(|(id,)| EntryId::from_str(id.as_str()).context("entry id from str"))
            .collect::<anyhow::Result<Vec<EntryId>>>()
    }

    pub async fn find_last_successful_indexing_started_at(
        &self,
    ) -> anyhow::Result<Option<Timestamp>> {
        let row: Option<(i64,)> = sqlx::query_as(
            r#"
SELECT MAX(indexings.at)
FROM indexings
INNER JOIN successful_indexings
ON successful_indexings.indexing_id = indexings.id
            "#,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|(at,)| Timestamp::from(at)))
    }

    pub async fn find_last_parsed_at(&self) -> anyhow::Result<Option<Timestamp>> {
        let row: Option<(i64,)> = sqlx::query_as(
            r#"
SELECT MAX(parsed_at)
FROM entries
            "#,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|(at,)| Timestamp::from(at)))
    }
}
