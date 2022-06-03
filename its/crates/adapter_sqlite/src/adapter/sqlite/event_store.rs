mod error;
mod event;
mod event_id;
mod event_row;
mod event_stream_id;
mod event_stream_row;
mod event_stream_seq;

pub use self::error::Error;
pub use self::event::Event;
pub use self::event_id::*;
use self::event_row::EventRow;
pub use self::event_stream_id::*;
use self::event_stream_row::EventStreamRow;
pub use self::event_stream_seq::*;

use sqlx::Transaction;
use sqlx::{any::AnyArguments, query::Query, Any};

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub async fn find_events_by_event_stream_id(
    transaction: &mut Transaction<'_, Any>,
    event_stream_id: EventStreamId,
) -> Result<Vec<Event>> {
    let event_rows: Vec<EventRow> = sqlx::query_as(include_str!(
        "../../../sql/command/select_events_by_event_stream_id.sql"
    ))
    .bind(event_stream_id.to_string())
    .fetch_all(&mut *transaction)
    .await?;
    Ok(event_rows.into_iter().map(Event::from).collect())
}

pub async fn find_events_by_event_stream_id_and_version_less_than_equal(
    transaction: &mut Transaction<'_, Any>,
    event_stream_id: EventStreamId,
    version: EventStreamSeq,
) -> Result<Vec<Event>> {
    let event_rows: Vec<EventRow> = sqlx::query_as(include_str!(
        "../../../sql/command/select_events_by_event_stream_id_and_version_less_than_equal.sql"
    ))
    .bind(event_stream_id.to_string())
    .bind(i64::from(version))
    .fetch_all(&mut *transaction)
    .await?;
    Ok(event_rows.into_iter().map(Event::from).collect())
}

pub async fn save(
    transaction: &mut Transaction<'_, Any>,
    current_version: Option<EventStreamSeq>,
    event: Event,
) -> Result<()> {
    if let Some(current_version) = current_version {
        let query: Query<Any, AnyArguments> =
            sqlx::query(include_str!("../../../sql/command/update_event_stream.sql"))
                .bind(i64::from(event.stream_seq))
                .bind(event.stream_id.to_string())
                .bind(i64::from(current_version));
        let result = query.execute(&mut *transaction).await?;
        if result.rows_affected() == 0 {
            return Err(Error::UpdateEventStream);
        }
    } else {
        let query: Query<Any, AnyArguments> =
            sqlx::query(include_str!("../../../sql/command/insert_event_stream.sql"))
                .bind(event.stream_id.to_string())
                .bind(i64::from(event.stream_seq));
        let result = query.execute(&mut *transaction).await?;
        if result.rows_affected() == 0 {
            return Err(Error::InsertEventStream);
        }
    }

    let query: Query<Any, AnyArguments> =
        sqlx::query(include_str!("../../../sql/command/insert_event.sql"))
            .bind(event.id.to_string())
            .bind(event.stream_id.to_string())
            .bind(i64::from(event.stream_seq))
            .bind(event.data);
    let result = query.execute(&mut *transaction).await?;
    if result.rows_affected() == 0 {
        return Err(Error::InsertEvent);
    }

    Ok(())
}

pub async fn find_event_stream_ids(
    transaction: &mut Transaction<'_, Any>,
) -> Result<Vec<EventStreamId>> {
    let event_stream_rows: Vec<EventStreamRow> = sqlx::query_as(include_str!(
        "../../../sql/command/select_event_streams.sql"
    ))
    .fetch_all(&mut *transaction)
    .await?;
    Ok(event_stream_rows.into_iter().map(|row| row.id()).collect())
}

pub async fn find_events(transaction: &mut Transaction<'_, Any>) -> Result<Vec<Event>> {
    let event_rows: Vec<EventRow> =
        sqlx::query_as(include_str!("../../../sql/command/select_events.sql"))
            .fetch_all(&mut *transaction)
            .await?;
    Ok(event_rows.into_iter().map(Event::from).collect())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use sqlx::{any::AnyConnectOptions, migrate::Migrator, AnyPool};

    use crate::{
        adapter::sqlite::{
            command_migration_source::CommandMigrationSource, event_store::event_id::EventId,
        },
        migrate1, migrate2,
    };

    use super::*;

    #[tokio::test]
    async fn read_and_write_test() -> anyhow::Result<()> {
        let options = AnyConnectOptions::from_str("sqlite::memory:")?;
        let pool = AnyPool::connect_with(options).await?;

        // FIXME: migration
        let iko_migrator = iko::Migrator::new(pool.clone());
        let mut iko_migrations = iko::Migrations::default();
        iko_migrations.push(1, migrate1)?;
        iko_migrations.push(2, migrate2)?;
        iko_migrator.migrate(&iko_migrations).await?;

        let migrator = Migrator::new(CommandMigrationSource::default()).await?;
        migrator.run(&pool).await?;

        let mut transaction = pool.begin().await?;

        let event_stream_ids = find_event_stream_ids(&mut transaction).await?;
        assert!(event_stream_ids.is_empty());

        let events = find_events(&mut transaction).await?;
        assert!(events.is_empty());

        let event_id = EventId::generate();
        let event_stream_id = EventStreamId::generate();
        let version = EventStreamSeq::from(1_u32);
        let create_event = Event {
            id: event_id,
            stream_id: event_stream_id,
            data: r#"{"type":"issue_created"}"#.to_string(),
            stream_seq: version,
        };
        save(&mut transaction, None, create_event).await?;

        transaction.commit().await?;
        let mut transaction = pool.begin().await?;

        // TODO: improve
        let event_stream_ids = find_event_stream_ids(&mut transaction).await?;
        assert!(!event_stream_ids.is_empty());
        assert_eq!(find_events(&mut transaction).await?.len(), 1);

        // TODO: improve
        let events = find_events_by_event_stream_id(&mut transaction, event_stream_id).await?;
        assert!(!events.is_empty());
        let events =
            find_events_by_event_stream_id(&mut transaction, EventStreamId::generate()).await?;
        assert!(events.is_empty());

        let event_id = EventId::generate();
        let update_event = Event {
            id: event_id,
            stream_id: event_stream_id,
            data: r#"{"type":"issue_updated"}"#.to_string(),
            stream_seq: EventStreamSeq::from(2_u32),
        };
        save(&mut transaction, Some(version), update_event).await?;
        transaction.commit().await?;

        let mut transaction = pool.begin().await?;
        assert_eq!(find_events(&mut transaction).await?.len(), 2);

        Ok(())
    }
}
