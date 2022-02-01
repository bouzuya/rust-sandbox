use std::{path::Path, str::FromStr};

use anyhow::Context;
use domain::{
    aggregate::{IssueAggregate, IssueAggregateEvent},
    IssueId,
};
use sqlx::{
    any::AnyConnectOptions,
    pool::PoolConnection,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    Any, AnyPool,
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

async fn connection(path: &Path) -> anyhow::Result<PoolConnection<Any>> {
    let options = SqliteConnectOptions::from_str(&format!(
        "sqlite:{}?mode=rwc",
        path.to_str().with_context(|| "invalid path")?
    ))?
    .journal_mode(SqliteJournalMode::Delete);
    let options = AnyConnectOptions::from(options);
    let pool = AnyPool::connect_with(options).await?;
    let conn = pool.acquire().await?;
    Ok(conn)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use sqlx::{
        any::{AnyArguments, AnyRow},
        Any, Arguments, Row,
    };
    use tempfile::tempdir;
    use ulid::Ulid;

    struct AggregateRow {
        id: Ulid,
        version: u64,
    }

    struct EventRow {
        aggregate_id: Ulid,
        data: String,
        version: u64,
    }

    struct EventStore {
        connection: PoolConnection<Any>,
    }

    fn row_to_aggregate_row(row: AnyRow) -> sqlx::Result<AggregateRow> {
        let version_as_i64: i64 = row.get("version");
        let version = u64::from_be_bytes(version_as_i64.to_be_bytes());
        let id: String = row.get("id");
        let id = Ulid::from_str(id.as_str()).unwrap();
        Ok(AggregateRow { id, version })
    }

    fn row_to_event_row(row: AnyRow) -> sqlx::Result<EventRow> {
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

        async fn save(
            &mut self,
            current_version: Option<u64>,
            event: EventRow,
        ) -> anyhow::Result<()> {
            if let Some(current_version) = current_version {
                let result = sqlx::query_with(include_str!("../../../sql/update_aggregate.sql"), {
                    let mut args = AnyArguments::default();
                    args.add(i64::from_be_bytes(event.version.to_be_bytes()));
                    args.add(event.aggregate_id.to_string());
                    args.add(i64::from_be_bytes(current_version.to_be_bytes()));
                    args
                })
                .execute(&mut self.connection)
                .await?;
                anyhow::ensure!(result.rows_affected() > 0, "update aggregate failed");
            } else {
                let result = sqlx::query_with(include_str!("../../../sql/insert_aggregate.sql"), {
                    let mut args = AnyArguments::default();
                    args.add(event.aggregate_id.to_string());
                    args.add(i64::from_be_bytes(event.version.to_be_bytes()));
                    args
                })
                .execute(&mut self.connection)
                .await?;
                anyhow::ensure!(result.rows_affected() > 0, "insert aggregate failed");
            }

            let result = sqlx::query_with(include_str!("../../../sql/insert_event.sql"), {
                let mut args = AnyArguments::default();
                args.add(event.aggregate_id.to_string());
                args.add(i64::from_be_bytes(event.version.to_be_bytes()));
                args.add(event.data);
                args
            })
            .execute(&mut self.connection)
            .await?;
            anyhow::ensure!(result.rows_affected() > 0, "insert event failed");

            Ok(())
        }

        async fn find_aggregates(&mut self) -> anyhow::Result<Vec<AggregateRow>> {
            Ok(
                sqlx::query(include_str!("../../../sql/select_aggregates.sql"))
                    .try_map(row_to_aggregate_row)
                    .fetch_all(&mut self.connection)
                    .await?,
            )
        }

        async fn find_events(&mut self) -> anyhow::Result<Vec<EventRow>> {
            Ok(sqlx::query(include_str!("../../../sql/select_events.sql"))
                .try_map(row_to_event_row)
                .fetch_all(&mut self.connection)
                .await?)
        }

        async fn find_events_by_aggregate_id(
            &mut self,
            aggregate_id: Ulid,
        ) -> anyhow::Result<Vec<EventRow>> {
            Ok(sqlx::query(include_str!(
                "../../../sql/select_events_by_aggregate_id.sql"
            ))
            .bind(aggregate_id.to_string())
            .try_map(row_to_event_row)
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

        let aggregate_id = Ulid::new();
        let version = 1;
        let data = r#"{"type":"issue_created"}"#.to_string();
        let event_row = EventRow {
            aggregate_id,
            data,
            version,
        };
        event_store.save(None, event_row).await?;

        // TODO: improve
        let aggregates = event_store.find_aggregates().await?;
        assert!(!aggregates.is_empty());
        assert_eq!(event_store.find_events().await?.len(), 1);

        // TODO: improve
        let aggregates = event_store
            .find_events_by_aggregate_id(aggregate_id)
            .await?;
        assert!(!aggregates.is_empty());
        let aggregates = event_store.find_events_by_aggregate_id(Ulid::new()).await?;
        assert!(aggregates.is_empty());

        let event_row = EventRow {
            aggregate_id,
            data: r#"{"type":"issue_updated"}"#.to_string(),
            version: 2,
        };
        event_store.save(Some(1), event_row).await?;
        assert_eq!(event_store.find_events().await?.len(), 2);

        Ok(())
    }
}
