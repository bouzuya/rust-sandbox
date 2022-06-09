use event_store::EventId;
use sqlx::{any::AnyArguments, query::Query, Any, AnyPool, Row};

pub async fn migrate2(
    pool: AnyPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut transaction = pool.begin().await?;

    sqlx::query(include_str!(
        "../../../../sql/command/migrations/20220603000001_create_tmp_events.sql"
    ))
    .execute(&mut transaction)
    .await?;

    let rows = sqlx::query(include_str!(
        "../../../../sql/command/migrations/20220603000002_select_old_events.sql"
    ))
    .fetch_all(&mut transaction)
    .await?;

    for row in rows {
        let query: Query<Any, AnyArguments> = sqlx::query(include_str!(
            "../../../../sql/command/migrations/20220603000003_insert_tmp_events.sql"
        ))
        .bind(EventId::generate().to_string())
        .bind(row.get::<'_, String, &str>("event_stream_id"))
        .bind(row.get::<'_, i64, &str>("version"))
        .bind(row.get::<'_, String, &str>("data"));
        query.execute(&mut transaction).await?;
    }

    sqlx::query(include_str!(
        "../../../../sql/command/migrations/20220603000004_rename_tmp_events_to_events.sql"
    ))
    .execute(&mut transaction)
    .await?;

    transaction.commit().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use event_store::EventStreamId;

    use crate::adapter::sqlite::migration::migrate1::migrate1;

    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let pool = AnyPool::connect("sqlite::memory:").await?;
        let iko_migrator = iko::Migrator::new(pool.clone());
        let mut iko_migrations = iko::Migrations::default();
        iko_migrations.push(1, migrate1)?;
        iko_migrator.migrate(&iko_migrations).await?;

        let mut transaction = pool.begin().await?;
        let event_stream_id = EventStreamId::generate();
        sqlx::query("INSERT INTO event_streams(id, version) VALUES(?, ?)")
            .bind(event_stream_id.to_string().as_str())
            .bind(1_i64)
            .execute(&mut transaction)
            .await?;
        sqlx::query("INSERT INTO events(event_stream_id, version, data) VALUES(?, ?, ?)")
            .bind(event_stream_id.to_string().as_str())
            .bind(1_i64)
            .bind("data")
            .execute(&mut transaction)
            .await?;
        transaction.commit().await?;

        iko_migrations.push(2, migrate2)?;
        iko_migrator.migrate(&iko_migrations).await?;

        let mut transaction = pool.begin().await?;

        let rows = sqlx::query("SELECT seq, id, event_stream_id, version, data FROM events")
            .fetch_all(&mut transaction)
            .await?;
        assert_eq!(rows[0].get::<'_, i64, _>("seq"), 0_i64);
        assert!(!rows[0].get::<'_, String, _>("id").is_empty());
        assert_eq!(
            rows[0].get::<'_, String, _>("event_stream_id"),
            event_stream_id.to_string()
        );
        assert_eq!(rows[0].get::<'_, i64, _>("version"), 1_i64);
        assert_eq!(rows[0].get::<'_, String, _>("data"), "data");

        Ok(())
    }
}
