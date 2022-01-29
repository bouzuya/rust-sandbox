use std::{path::Path, str::FromStr};

use anyhow::Context;
use domain::{
    aggregate::{IssueAggregate, IssueAggregateEvent},
    IssueId,
};
use sqlx::{
    pool::PoolConnection,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    Sqlite, SqlitePool,
};
use use_case::{IssueRepository, RepositoryError};

#[derive(Debug, Default)]
pub struct SqliteIssueRepository {}

impl IssueRepository for SqliteIssueRepository {
    fn find_by_id(&self, issue_id: &IssueId) -> Result<Option<IssueAggregate>, RepositoryError> {
        todo!()
    }

    fn last_created(&self) -> Result<Option<IssueAggregate>, RepositoryError> {
        todo!()
    }

    fn save(&self, event: IssueAggregateEvent) -> Result<(), RepositoryError> {
        todo!()
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use sqlx::{sqlite::SqliteRow, Row};
    use tempfile::tempdir;
    use ulid::Ulid;

    struct AggregateRow {
        id: Ulid,
        version: u64,
        r#type: String,
    }

    struct EventRow {
        aggregate_id: Ulid,
        data: String,
        version: u64,
    }

    struct EventStore {
        connection: PoolConnection<Sqlite>,
    }

    impl EventStore {
        async fn new(path_buf: PathBuf) -> anyhow::Result<Self> {
            let mut conn = connection(path_buf.as_path()).await?;

            // migrate
            sqlx::query(include_str!("../../../sql/create_aggregates.sql"))
                .execute(&mut conn)
                .await?;
            sqlx::query(include_str!("../../../sql/create_events.sql"))
                .execute(&mut conn)
                .await?;

            Ok(Self { connection: conn })
        }

        async fn find_aggregates(&mut self) -> anyhow::Result<Vec<AggregateRow>> {
            Ok(
                sqlx::query(include_str!("../../../sql/select_aggregates.sql"))
                    .try_map(|row: SqliteRow| {
                        let r#type: String = row.get("type");
                        let version_as_i64: i64 = row.get("version");
                        let version = u64::from_be_bytes(version_as_i64.to_be_bytes());
                        let id: String = row.get("id");
                        let id = Ulid::from_str(id.as_str()).unwrap();
                        Ok(AggregateRow {
                            id,
                            version,
                            r#type,
                        })
                    })
                    .fetch_all(&mut self.connection)
                    .await?,
            )
        }

        async fn find_events(&mut self) -> anyhow::Result<Vec<EventRow>> {
            Ok(sqlx::query(include_str!("../../../sql/select_events.sql"))
                .try_map(|row: SqliteRow| {
                    let data: String = row.get("data");
                    let version_as_i64: i64 = row.get("version");
                    let version = u64::from_be_bytes(version_as_i64.to_be_bytes());
                    let aggregate_id: String = row.get("aggregate_id");
                    let aggregate_id = Ulid::from_str(aggregate_id.as_str()).unwrap();
                    Ok(EventRow {
                        aggregate_id,
                        data,
                        version,
                    })
                })
                .fetch_all(&mut self.connection)
                .await?)
        }
    }

    #[tokio::test]
    async fn read_and_write_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let mut event_store = EventStore::new(temp_dir.path().join("its.sqlite")).await?;

        let aggregates = event_store.find_aggregates().await?;
        assert!(aggregates.is_empty());

        let events = event_store.find_events().await?;
        assert!(events.is_empty());

        Ok(())
    }
}
