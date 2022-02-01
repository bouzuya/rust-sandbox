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
    use std::{fmt::Display, path::PathBuf};

    use super::*;
    use sqlx::{
        any::{AnyArguments, AnyRow},
        Any, Arguments, FromRow, Row,
    };
    use tempfile::tempdir;
    use ulid::Ulid;

    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub struct AggregateId(Ulid);

    impl AggregateId {
        pub fn generate() -> Self {
            Self(Ulid::new())
        }
    }

    impl Display for AggregateId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl FromStr for AggregateId {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let ulid = Ulid::from_str(s)?;
            Ok(Self(ulid))
        }
    }

    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub struct AggregateVersion(u32);

    impl From<AggregateVersion> for i64 {
        fn from(version: AggregateVersion) -> Self {
            i64::from(version.0)
        }
    }

    impl From<u32> for AggregateVersion {
        fn from(value: u32) -> Self {
            Self(value)
        }
    }

    impl TryFrom<i64> for AggregateVersion {
        type Error = anyhow::Error;

        fn try_from(value: i64) -> Result<Self, Self::Error> {
            let value = u32::try_from(value)?;
            Ok(Self(value))
        }
    }

    #[derive(Debug)]
    struct AggregateRow {
        id: String,
        version: i64,
    }

    impl<'r> FromRow<'r, AnyRow> for AggregateRow {
        fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
            Ok(Self {
                id: row.get("id"),
                version: row.get("version"),
            })
        }
    }

    #[derive(Debug)]
    struct EventRow {
        aggregate_id: String,
        data: String,
        version: i64,
    }

    impl<'r> FromRow<'r, AnyRow> for EventRow {
        fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
            Ok(Self {
                aggregate_id: row.get("aggregate_id"),
                data: row.get("data"),
                version: row.get("version"),
            })
        }
    }

    struct EventStore {
        connection: PoolConnection<Any>,
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
            current_version: Option<AggregateVersion>,
            event: EventRow,
        ) -> anyhow::Result<()> {
            if let Some(current_version) = current_version {
                let result = sqlx::query_with(include_str!("../../../sql/update_aggregate.sql"), {
                    let mut args = AnyArguments::default();
                    args.add(i64::from_be_bytes(event.version.to_be_bytes()));
                    args.add(event.aggregate_id.to_string());
                    args.add(i64::from(current_version));
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
                sqlx::query_as(include_str!("../../../sql/select_aggregates.sql"))
                    .fetch_all(&mut self.connection)
                    .await?,
            )
        }

        async fn find_events(&mut self) -> anyhow::Result<Vec<EventRow>> {
            Ok(
                sqlx::query_as(include_str!("../../../sql/select_events.sql"))
                    .fetch_all(&mut self.connection)
                    .await?,
            )
        }

        async fn find_events_by_aggregate_id(
            &mut self,
            aggregate_id: AggregateId,
        ) -> anyhow::Result<Vec<EventRow>> {
            Ok(sqlx::query_as(include_str!(
                "../../../sql/select_events_by_aggregate_id.sql"
            ))
            .bind(aggregate_id.to_string())
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

        let aggregate_id = AggregateId::generate();
        let version = AggregateVersion::from(1_u32);
        let data = r#"{"type":"issue_created"}"#.to_string();
        let event_row = EventRow {
            aggregate_id: aggregate_id.to_string(),
            data,
            version: i64::from(version),
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
        let aggregates = event_store
            .find_events_by_aggregate_id(AggregateId::generate())
            .await?;
        assert!(aggregates.is_empty());

        let event_row = EventRow {
            aggregate_id: aggregate_id.to_string(),
            data: r#"{"type":"issue_updated"}"#.to_string(),
            version: 2,
        };
        event_store.save(Some(version), event_row).await?;
        assert_eq!(event_store.find_events().await?.len(), 2);

        Ok(())
    }
}
