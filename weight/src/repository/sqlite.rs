use super::EventRepository;
use crate::{event::Event, remove::Remove, set::Set};
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
    Ok(sqlx::query(
        r#"
        SELECT
          event.id AS event_id,
          event.type AS event_type,
          remove_event.key AS remove_key,
          set_event.key AS set_key,
          set_event.value AS set_value
        FROM events AS event
        LEFT OUTER JOIN remove_events AS remove_event ON event.id = remove_event.id
        LEFT OUTER JOIN set_events AS set_event ON event.id = set_event.id
        ORDER BY event.id ASC
        "#,
    )
    .try_map(|row: SqliteRow| {
        let t: String = row.get("event_type");
        match t.as_str() {
            "remove" => Ok(Event::Remove(Remove::new(row.get("remove_key")))),
            "set" => Set::new(row.get("set_key"), row.get("set_value"))
                .map(|s| Event::Set(s))
                .map_err(|e| sqlx::Error::Decode(Box::new(e))),
            _ => unreachable!(),
        }
    })
    .fetch_all(&mut conn)
    .await?)
}

async fn write_sqlite(path: &Path, events: &Vec<Event>) -> anyhow::Result<()> {
    let mut conn = connection(path).await?;

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    type TEXT NOT NULL
)"#,
    )
    .execute(&mut conn)
    .await?;

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS remove_events (
    id INTEGER PRIMARY KEY,
    key TEXT NOT NULL,
    FOREIGN KEY (id) REFERENCES events (id)
)"#,
    )
    .execute(&mut conn)
    .await?;

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS set_events (
    id INTEGER PRIMARY KEY,
    key TEXT NOT NULL,
    value REAL NOT NULL,
    FOREIGN KEY (id) REFERENCES events (id)
)"#,
    )
    .execute(&mut conn)
    .await?;

    for event in events {
        match event {
            Event::Set(set) => {
                let id = sqlx::query("INSERT INTO events (type) VALUES (?)")
                    .bind("set")
                    .execute(&mut conn)
                    .await?
                    .last_insert_rowid();
                sqlx::query("INSERT INTO set_events (id, key, value) VALUES (?, ?, ?)")
                    .bind(id)
                    .bind(set.key())
                    .bind(set.value())
                    .execute(&mut conn)
                    .await?;
            }
            Event::Remove(remove) => {
                let id = sqlx::query("INSERT INTO events (type) VALUES (?)")
                    .bind("remove")
                    .execute(&mut conn)
                    .await?
                    .last_insert_rowid();
                sqlx::query("INSERT INTO remove_events (id, key) VALUES (?, ?)")
                    .bind(id)
                    .bind(remove.key())
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
            Event::Remove(Remove::new("2021-03-04".to_string())),
        ];
        write_sqlite(sqlite.as_path(), &events).await.unwrap();
        assert_eq!(read_sqlite(sqlite.as_path()).await.unwrap(), events);
    }
}
