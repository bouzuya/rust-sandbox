use sqlx::AnyPool;

pub async fn migrate3(
    pool: AnyPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut transaction = pool.begin().await?;

    sqlx::query(include_str!(
        "../../../../sql/command/migrations/20220806000001_create_issue_comment_ids.sql"
    ))
    .execute(&mut transaction)
    .await?;

    transaction.commit().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use domain::IssueCommentId;
    use event_store::{EventId, EventStreamId};
    use sqlx::Row;

    use crate::adapter::sqlite::migration::{migrate1, migrate2};

    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let pool = AnyPool::connect("sqlite::memory:").await?;
        let iko_migrator = iko::Migrator::new(pool.clone());
        let mut iko_migrations = iko::Migrations::default();
        iko_migrations.push(1, migrate1)?;
        iko_migrations.push(2, migrate2)?;
        iko_migrations.push(3, migrate3)?;
        iko_migrator.migrate(&iko_migrations).await?;

        let mut transaction = pool.begin().await?;
        let issue_comment_id = IssueCommentId::generate();
        let event_id = EventId::generate();
        let event_stream_id = EventStreamId::generate();
        sqlx::query("INSERT INTO event_streams(id, version) VALUES(?, ?)")
            .bind(event_stream_id.to_string().as_str())
            .bind(1_i64)
            .execute(&mut transaction)
            .await?;
        sqlx::query("INSERT INTO events(id, event_stream_id, version, data) VALUES(?, ?, ?, ?)")
            .bind(event_id.to_string().as_str())
            .bind(event_stream_id.to_string().as_str())
            .bind(1_i64)
            .bind("data")
            .execute(&mut transaction)
            .await?;
        sqlx::query(
            "INSERT INTO issue_comment_ids(issue_comment_id, event_stream_id) VALUES(?, ?)",
        )
        .bind(issue_comment_id.to_string().as_str())
        .bind(event_stream_id.to_string().as_str())
        .execute(&mut transaction)
        .await?;
        transaction.commit().await?;

        let mut transaction = pool.begin().await?;

        let rows = sqlx::query("SELECT issue_comment_id, event_stream_id FROM issue_comment_ids")
            .fetch_all(&mut transaction)
            .await?;
        assert_eq!(
            rows[0].get::<'_, String, _>("issue_comment_id"),
            issue_comment_id.to_string()
        );
        assert_eq!(
            rows[0].get::<'_, String, _>("event_stream_id"),
            event_stream_id.to_string()
        );

        Ok(())
    }
}
