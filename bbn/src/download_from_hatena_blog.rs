use std::{collections::BTreeSet, convert::TryInto, path::PathBuf, str::FromStr, time::Duration};

use anyhow::Context as _;
use hatena_blog::{Client, Config, EntryId};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
    Pool, Sqlite,
};
use tokio::time::sleep;

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
        sqlx::query("CREATE TABLE IF NOT EXISTS entry_ids (entry_id TEXT PRIMARY KEY)")
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
        let response = client.list_entries_in_page(next_page.as_deref()).await?;
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
