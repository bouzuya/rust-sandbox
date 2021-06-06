use super::EventRepository;
use crate::{event::Event, set::Set};
use anyhow::Context;
use async_trait::async_trait;
use sqlx::{
    pool::PoolConnection,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteRow},
    Row, Sqlite, SqlitePool,
};
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

pub struct SqliteEventRepository {
    data_file: PathBuf,
}

impl SqliteEventRepository {
    pub fn new(data_file: PathBuf) -> Self {
        Self { data_file }
    }
}

#[async_trait]
impl EventRepository for SqliteEventRepository {
    async fn find_all(&self) -> anyhow::Result<Vec<Event>> {
        Ok(read_sqlite(self.data_file.as_path()).await?)
    }

    async fn save(&self, events: &Vec<Event>) -> anyhow::Result<()> {
        Ok(write_sqlite(self.data_file.as_path(), events).await?)
    }
}

async fn connection(path: &Path) -> anyhow::Result<PoolConnection<Sqlite>> {
    let options = SqliteConnectOptions::from_str(&format!(
        "sqlite:{}?mode=rwc",
        path.to_str().with_context(|| "invalid path")?
    ))?
    .journal_mode(SqliteJournalMode::Delete);
    let pool = SqlitePool::connect_with(options).await?;
    let conn = pool.acquire().await?;
    Ok(conn)
}

async fn read_sqlite(path: &Path) -> anyhow::Result<Vec<Event>> {
    if !path.exists() {
        return Ok(vec![]);
    }

    let mut conn = connection(path).await?;
    Ok(
        sqlx::query("SELECT id, key, value FROM events ORDER BY id ASC")
            .try_map(|row: SqliteRow| {
                Set::new(row.get("key"), row.get("value"))
                    .map(|s| Event::Set(s))
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))
            })
            .fetch_all(&mut conn)
            .await?,
    )
}

async fn write_sqlite(path: &Path, events: &Vec<Event>) -> anyhow::Result<()> {
    let mut conn = connection(path).await?;

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT NOT NULL,
    value REAL NOT NULL
)"#,
    )
    .execute(&mut conn)
    .await?;

    for event in events {
        match event {
            Event::Set(set) => {
                sqlx::query("INSERT INTO events (key, value) VALUES (?, ?)")
                    .bind(set.key())
                    .bind(set.value())
                    .execute(&mut conn)
                    .await?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[async_std::test]
    async fn read_and_write_test() {
        let dir = tempdir().unwrap();
        let sqlite = dir.path().join("weight.sqlite");

        // not exists
        assert_eq!(read_sqlite(sqlite.as_path()).await.unwrap(), vec![]);

        // not file
        fs::create_dir(sqlite.as_path()).unwrap();
        assert_eq!(read_sqlite(sqlite.as_path()).await.is_err(), true);
        fs::remove_dir(sqlite.as_path()).unwrap();

        // broken sqlite db
        fs::write(sqlite.as_path(), concat!(r#"{]"#, "\n",)).unwrap();
        assert_eq!(read_sqlite(sqlite.as_path()).await.is_err(), true);
        fs::remove_file(sqlite.as_path()).unwrap();

        // convert error (can't test)

        // OK
        let events = vec![
            Event::Set(Set::new("2021-02-03".to_string(), 50.1).unwrap()),
            Event::Set(Set::new("2021-03-04".to_string(), 51.2).unwrap()),
        ];
        write_sqlite(sqlite.as_path(), &events).await.unwrap();
        assert_eq!(read_sqlite(sqlite.as_path()).await.unwrap(), events);
    }
}
