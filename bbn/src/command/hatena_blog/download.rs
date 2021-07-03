use std::{collections::BTreeSet, convert::TryInto, path::PathBuf, str::FromStr, time::Duration};

use anyhow::Context as _;
use hatena_blog::{Client, Config, Entry, EntryId};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
    Pool, Sqlite,
};
use tokio::time::sleep;

use crate::timestamp::Timestamp;

#[derive(Debug)]
struct Repository {
    data_file: PathBuf,
    pool: Pool<Sqlite>,
}

impl Repository {
    async fn new(data_file: PathBuf) -> anyhow::Result<Self> {
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
        CREATE TABLE IF NOT EXISTS list_requests (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            at INTEGER NOT NULL,
            page TEXT
        )"#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
        CREATE TABLE IF NOT EXISTS list_responses (
            request_id INTEGER PRIMARY KEY,
            body TEXT NOT NULL,
            FOREIGN KEY (request_id) REFERENCES list_requests(id)
        )"#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
        CREATE TABLE IF NOT EXISTS last_list_request_at (
            at INTEGER
        )"#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
        CREATE TABLE IF NOT EXISTS member_responses (
            entry_id TEXT PRIMARY KEY,
            at TIMESTAMP NOT NULL,
            body TEXT NOT NULL,
            FOREIGN KEY (entry_id) REFERENCES entry_ids(entry_id)
        )"#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { data_file, pool })
    }

    async fn add(
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

    async fn get_last_list_request_at(&self) -> anyhow::Result<Option<Timestamp>> {
        let row: Option<(i64,)> = sqlx::query_as(
            r#"
SELECT at FROM last_list_request_at
            "#,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|(at,)| Timestamp::from(at)))
    }

    async fn set_last_list_request_at(&self, at: Timestamp) -> anyhow::Result<()> {
        let result = sqlx::query(
            r#"
UPDATE last_list_request_at SET at=?
            "#,
        )
        .bind(i64::from(at))
        .execute(&self.pool)
        .await?;
        let count = result.rows_affected();
        if count == 0 {
            sqlx::query(
                r#"
INSERT INTO last_list_request_at(at) VALUES (?)
            "#,
            )
            .bind(i64::from(at))
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }

    async fn create_list_request(
        &self,
        at: Timestamp,
        page: &Option<String>,
    ) -> anyhow::Result<i64> {
        Ok(sqlx::query(
            r#"
            INSERT INTO list_requests(at, page)
            VALUES (?, ?)
            "#,
        )
        .bind(i64::from(at))
        .bind(page)
        .execute(&self.pool)
        .await?
        .last_insert_rowid())
    }

    async fn create_list_response(&self, request_id: i64, body: String) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO list_responses(request_id, body)
            VALUES (?, ?)
            "#,
        )
        .bind(request_id)
        .bind(body)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn create_member_response(
        &self,
        entry_id: &EntryId,
        at: Timestamp,
        body: String,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO member_responses(entry_id, at, body)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(entry_id.to_string())
        .bind(i64::from(at))
        .bind(body)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_incomplete_entry_ids(&self) -> anyhow::Result<Vec<EntryId>> {
        let rows: Vec<(String,)> = sqlx::query_as(
            r#"
SELECT entry_id
  FROM entry_ids
  LEFT OUTER JOIN member_responses USING(entry_id)
 WHERE member_responses.entry_id IS NULL
 ORDER BY published ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;
        rows.iter()
            .map(|(id,)| EntryId::from_str(id.as_str()).context("entry id from str"))
            .collect::<anyhow::Result<Vec<EntryId>>>()
    }
}

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
