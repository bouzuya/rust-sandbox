use sqlx::{migrate::Migrator, AnyPool};

use crate::adapter::sqlite::command_migration_source::CommandMigrationSource;

pub async fn migrate1(
    pool: AnyPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let migrator = Migrator::new(CommandMigrationSource::default()).await?;
    migrator.run(&pool).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let pool = AnyPool::connect("sqlite::memory:").await?;
        let iko_migrator = iko::Migrator::new(pool.clone());
        let mut iko_migrations = iko::Migrations::default();
        iko_migrations.push(1, migrate1)?;
        iko_migrator.migrate(&iko_migrations).await?;

        let mut transaction = pool.begin().await?;

        let rows = sqlx::query("SELECT id, version FROM event_streams")
            .fetch_all(&mut transaction)
            .await?;
        assert!(rows.is_empty());

        let rows = sqlx::query("SELECT event_stream_id, version, data FROM events")
            .fetch_all(&mut transaction)
            .await?;
        assert!(rows.is_empty());

        let rows = sqlx::query("SELECT issue_number, event_stream_id FROM issue_ids")
            .fetch_all(&mut transaction)
            .await?;
        assert!(rows.is_empty());

        let rows =
            sqlx::query("SELECT issue_block_link_id, event_stream_id FROM issue_block_link_ids")
                .fetch_all(&mut transaction)
                .await?;
        assert!(rows.is_empty());
        Ok(())
    }
}
