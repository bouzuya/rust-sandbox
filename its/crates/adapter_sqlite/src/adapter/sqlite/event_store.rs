mod aggregate_id;
mod aggregate_row;
mod aggregate_version;
mod event;
mod event_row;
mod event_store_error;

pub use self::aggregate_id::*;
use self::aggregate_row::AggregateRow;
pub use self::aggregate_version::*;
pub use self::event::Event;
use self::event_row::EventRow;
use self::event_store_error::EventStoreError;

use sqlx::Transaction;
use sqlx::{any::AnyArguments, query::Query, Any};

pub async fn find_events_by_aggregate_id(
    transaction: &mut Transaction<'_, Any>,
    aggregate_id: AggregateId,
) -> Result<Vec<Event>, EventStoreError> {
    let event_rows: Vec<EventRow> = sqlx::query_as(include_str!(
        "../../../sql/command/select_events_by_aggregate_id.sql"
    ))
    .bind(aggregate_id.to_string())
    .fetch_all(&mut *transaction)
    .await?;
    Ok(event_rows.into_iter().map(Event::from).collect())
}

pub async fn find_events_by_aggregate_id_and_version_less_than_equal(
    transaction: &mut Transaction<'_, Any>,
    aggregate_id: AggregateId,
    version: AggregateVersion,
) -> Result<Vec<Event>, EventStoreError> {
    let event_rows: Vec<EventRow> = sqlx::query_as(include_str!(
        "../../../sql/command/select_events_by_aggregate_id_and_version_less_than_equal.sql"
    ))
    .bind(aggregate_id.to_string())
    .bind(i64::from(version))
    .fetch_all(&mut *transaction)
    .await?;
    Ok(event_rows.into_iter().map(Event::from).collect())
}

pub async fn save(
    transaction: &mut Transaction<'_, Any>,
    current_version: Option<AggregateVersion>,
    event: Event,
) -> Result<(), EventStoreError> {
    if let Some(current_version) = current_version {
        let query: Query<Any, AnyArguments> =
            sqlx::query(include_str!("../../../sql/command/update_aggregate.sql"))
                .bind(i64::from(event.version))
                .bind(event.aggregate_id.to_string())
                .bind(i64::from(current_version));
        let result = query.execute(&mut *transaction).await?;
        if result.rows_affected() == 0 {
            return Err(EventStoreError::UpdateAggregate);
        }
    } else {
        let query: Query<Any, AnyArguments> =
            sqlx::query(include_str!("../../../sql/command/insert_aggregate.sql"))
                .bind(event.aggregate_id.to_string())
                .bind(i64::from(event.version));
        let result = query.execute(&mut *transaction).await?;
        if result.rows_affected() == 0 {
            return Err(EventStoreError::InsertAggregate);
        }
    }

    let query: Query<Any, AnyArguments> =
        sqlx::query(include_str!("../../../sql/command/insert_event.sql"))
            .bind(event.aggregate_id.to_string())
            .bind(i64::from(event.version))
            .bind(event.data);
    let result = query.execute(&mut *transaction).await?;
    if result.rows_affected() == 0 {
        return Err(EventStoreError::InsertEvent);
    }

    Ok(())
}

pub async fn find_aggregate_ids(
    transaction: &mut Transaction<'_, Any>,
) -> Result<Vec<AggregateId>, EventStoreError> {
    let aggregate_rows: Vec<AggregateRow> =
        sqlx::query_as(include_str!("../../../sql/command/select_aggregates.sql"))
            .fetch_all(&mut *transaction)
            .await?;
    Ok(aggregate_rows.into_iter().map(|row| row.id()).collect())
}

pub async fn find_events(
    transaction: &mut Transaction<'_, Any>,
) -> Result<Vec<Event>, EventStoreError> {
    let event_rows: Vec<EventRow> =
        sqlx::query_as(include_str!("../../../sql/command/select_events.sql"))
            .fetch_all(&mut *transaction)
            .await?;
    Ok(event_rows.into_iter().map(Event::from).collect())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anyhow::Context;
    use sqlx::{
        any::AnyConnectOptions,
        migrate::Migrator,
        sqlite::{SqliteConnectOptions, SqliteJournalMode},
        AnyPool,
    };
    use tempfile::tempdir;

    use crate::adapter::sqlite::command_migration_source::CommandMigrationSource;

    use super::*;

    #[tokio::test]
    async fn read_and_write_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let sqlite_path = temp_dir.path().join("command.sqlite");
        let path = sqlite_path.as_path();
        let options = SqliteConnectOptions::from_str(&format!(
            "sqlite:{}?mode=rwc",
            path.to_str().with_context(|| "invalid path")?
        ))?
        .journal_mode(SqliteJournalMode::Delete);
        let options = AnyConnectOptions::from(options);
        let pool = AnyPool::connect_with(options).await?;

        let migrator = Migrator::new(CommandMigrationSource::default()).await?;
        migrator.run(&pool).await?;

        let mut transaction = pool.begin().await?;

        let aggregates = find_aggregate_ids(&mut transaction).await?;
        assert!(aggregates.is_empty());

        let events = find_events(&mut transaction).await?;
        assert!(events.is_empty());

        let aggregate_id = AggregateId::generate();
        let version = AggregateVersion::from(1_u32);
        let create_event = Event {
            aggregate_id,
            data: r#"{"type":"issue_created"}"#.to_string(),
            version,
        };
        save(&mut transaction, None, create_event).await?;

        transaction.commit().await?;
        let mut transaction = pool.begin().await?;

        // TODO: improve
        let aggregates = find_aggregate_ids(&mut transaction).await?;
        assert!(!aggregates.is_empty());
        assert_eq!(find_events(&mut transaction).await?.len(), 1);

        // TODO: improve
        let aggregates = find_events_by_aggregate_id(&mut transaction, aggregate_id).await?;
        assert!(!aggregates.is_empty());
        let aggregates =
            find_events_by_aggregate_id(&mut transaction, AggregateId::generate()).await?;
        assert!(aggregates.is_empty());

        let update_event = Event {
            aggregate_id,
            data: r#"{"type":"issue_updated"}"#.to_string(),
            version: AggregateVersion::from(2_u32),
        };
        save(&mut transaction, Some(version), update_event).await?;
        transaction.commit().await?;

        let mut transaction = pool.begin().await?;
        assert_eq!(find_events(&mut transaction).await?.len(), 2);

        Ok(())
    }
}
