mod aggregate_id;
mod aggregate_version;
mod event;
mod event_store_error;

pub use self::aggregate_id::*;
pub use self::aggregate_version::*;
pub use self::event::Event;
use self::event_store_error::EventStoreError;
use std::path::PathBuf;
use std::{path::Path, str::FromStr};

use anyhow::Context;
use sqlx::Transaction;
use sqlx::{
    any::{AnyArguments, AnyRow},
    query::Query,
    Any, FromRow, Row,
};

use sqlx::{
    any::AnyConnectOptions,
    pool::PoolConnection,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    AnyPool,
};

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

impl From<EventRow> for Event {
    fn from(row: EventRow) -> Self {
        Self {
            aggregate_id: AggregateId::from_str(row.aggregate_id.as_str())
                .expect("invalid aggregate_id"),
            data: row.data,
            version: AggregateVersion::try_from(row.version).expect("invalid version"),
        }
    }
}

pub struct EventStore {
    connection: PoolConnection<Any>,
}

impl EventStore {
    pub async fn new(path_buf: PathBuf) -> Result<Self, EventStoreError> {
        let mut conn = Self::connection(path_buf.as_path())
            .await
            .map_err(|_| EventStoreError::IO)?;

        // migrate
        sqlx::query(include_str!("../../../sql/create_aggregates.sql"))
            .execute(&mut conn)
            .await
            .map_err(|_| EventStoreError::MigrateCreateAggregateTable)?;
        sqlx::query(include_str!("../../../sql/create_events.sql"))
            .execute(&mut conn)
            .await
            .map_err(|_| EventStoreError::MigrateCreateEventTable)?;

        Ok(Self { connection: conn })
    }

    pub async fn save(
        transaction: &mut Transaction<'_, Any>,
        current_version: Option<AggregateVersion>,
        event: Event,
    ) -> Result<(), EventStoreError> {
        if let Some(current_version) = current_version {
            let query: Query<Any, AnyArguments> =
                sqlx::query(include_str!("../../../sql/update_aggregate.sql"))
                    .bind(i64::from(event.version))
                    .bind(event.aggregate_id.to_string())
                    .bind(i64::from(current_version));
            let result = query
                .execute(&mut *transaction)
                .await
                .map_err(|_| EventStoreError::IO)?;
            if result.rows_affected() == 0 {
                return Err(EventStoreError::UpdateAggregate);
            }
        } else {
            let query: Query<Any, AnyArguments> =
                sqlx::query(include_str!("../../../sql/insert_aggregate.sql"))
                    .bind(event.aggregate_id.to_string())
                    .bind(i64::from(event.version));
            let result = query
                .execute(&mut *transaction)
                .await
                .map_err(|_| EventStoreError::IO)?;
            if result.rows_affected() == 0 {
                return Err(EventStoreError::InsertAggregate);
            }
        }

        let query: Query<Any, AnyArguments> =
            sqlx::query(include_str!("../../../sql/insert_event.sql"))
                .bind(event.aggregate_id.to_string())
                .bind(i64::from(event.version))
                .bind(event.data);
        let result = query
            .execute(&mut *transaction)
            .await
            .map_err(|_| EventStoreError::IO)?;
        if result.rows_affected() == 0 {
            return Err(EventStoreError::InsertEvent);
        }

        Ok(())
    }

    pub async fn find_aggregate_ids(&mut self) -> Result<Vec<AggregateId>, EventStoreError> {
        let aggregate_rows: Vec<AggregateRow> =
            sqlx::query_as(include_str!("../../../sql/select_aggregates.sql"))
                .fetch_all(&mut self.connection)
                .await
                .map_err(|_| EventStoreError::IO)?;
        aggregate_rows
            .into_iter()
            .map(|row| AggregateId::from_str(row.id.as_str()))
            .collect()
    }

    pub async fn find_events(&mut self) -> Result<Vec<Event>, EventStoreError> {
        let event_rows: Vec<EventRow> =
            sqlx::query_as(include_str!("../../../sql/select_events.sql"))
                .fetch_all(&mut self.connection)
                .await
                .map_err(|_| EventStoreError::IO)?;
        Ok(event_rows.into_iter().map(Event::from).collect())
    }

    pub async fn find_events_by_aggregate_id(
        &mut self,
        aggregate_id: AggregateId,
    ) -> Result<Vec<Event>, EventStoreError> {
        let event_rows: Vec<EventRow> = sqlx::query_as(include_str!(
            "../../../sql/select_events_by_aggregate_id.sql"
        ))
        .bind(aggregate_id.to_string())
        .fetch_all(&mut self.connection)
        .await
        .map_err(|_| EventStoreError::IO)?;
        Ok(event_rows.into_iter().map(Event::from).collect())
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
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[tokio::test]
    async fn read_and_write_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let sqlite_path = temp_dir.path().join("its.sqlite");
        let mut event_store = EventStore::new(sqlite_path.clone()).await?;

        let path = sqlite_path.as_path();
        let options = SqliteConnectOptions::from_str(&format!(
            "sqlite:{}?mode=rwc",
            path.to_str().with_context(|| "invalid path")?
        ))?
        .journal_mode(SqliteJournalMode::Delete);
        let options = AnyConnectOptions::from(options);
        let pool = AnyPool::connect_with(options).await?;
        let mut transaction = pool.begin().await?;

        let aggregates = event_store.find_aggregate_ids().await?;
        assert!(aggregates.is_empty());

        let events = event_store.find_events().await?;
        assert!(events.is_empty());

        let aggregate_id = AggregateId::generate();
        let version = AggregateVersion::from(1_u32);
        let create_event = Event {
            aggregate_id,
            data: r#"{"type":"issue_created"}"#.to_string(),
            version,
        };
        EventStore::save(&mut transaction, None, create_event).await?;

        transaction.commit().await?;
        let mut transaction = pool.begin().await?;

        // TODO: improve
        let aggregates = event_store.find_aggregate_ids().await?;
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

        let update_event = Event {
            aggregate_id,
            data: r#"{"type":"issue_updated"}"#.to_string(),
            version: AggregateVersion::from(2_u32),
        };
        EventStore::save(&mut transaction, Some(version), update_event).await?;
        transaction.commit().await?;
        assert_eq!(event_store.find_events().await?.len(), 2);

        Ok(())
    }
}
