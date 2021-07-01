use std::{
    collections::BTreeSet,
    convert::TryInto,
    path::PathBuf,
    str::FromStr,
    time::{Duration, SystemTime},
};

use anyhow::Context as _;
use hatena_blog::{Client, Config, EntryId};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
    Pool, Sqlite,
};
use tokio::time::sleep;

fn now() -> anyhow::Result<i64> {
    Ok(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs()
        .try_into()?)
}

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
            entry_id TEXT PRIMARY KEY
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
        Ok(Self { data_file, pool })
    }

    async fn add(&self, entry_id: &EntryId) -> anyhow::Result<()> {
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM entry_ids WHERE entry_id = ?")
            .bind(entry_id.to_string())
            .fetch_one(&self.pool)
            .await?;
        if count > 0 {
            return Ok(());
        }
        Ok(sqlx::query("INSERT INTO entry_ids VALUES (?)")
            .bind(entry_id.to_string())
            .execute(&self.pool)
            .await
            .map(|_| ())?)
    }

    async fn create_list_request(&self, at: i64, page: &Option<String>) -> anyhow::Result<i64> {
        Ok(sqlx::query(
            r#"
            INSERT INTO list_requests(at, page)
            VALUES (?, ?)
            "#,
        )
        .bind(at)
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

    let mut set = BTreeSet::new();
    let mut next_page = None;
    loop {
        let at = now()?;
        let request_id = repository.create_list_request(at, &next_page).await?;
        let response = client.list_entries_in_page(next_page.as_deref()).await?;
        let body = response.to_string();
        repository.create_list_response(request_id, body).await?;
        let (next, entry_ids) = response.try_into()?;
        for entry_id in entry_ids {
            set.insert(entry_id.to_string());
            repository.add(&entry_id).await?;
        }
        match next {
            None => {
                break;
            }
            Some(page) => {
                next_page = Some(page);
            }
        }
        sleep(Duration::from_secs(1)).await;
    }

    for entry_id in set {
        println!("{}", entry_id);
    }

    Ok(())
}
