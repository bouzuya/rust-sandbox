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
    use super::*;
    use sqlx::{sqlite::SqliteRow, Row};
    use tempfile::tempdir;

    #[tokio::test]
    async fn read_and_write_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let sqlite = temp_dir.path().join("its.sqlite");

        let mut conn = connection(sqlite.as_path()).await?;

        // migrate
        sqlx::query(include_str!("../../../sql/create_aggregates.sql"))
            .execute(&mut conn)
            .await?;
        sqlx::query(include_str!("../../../sql/create_events.sql"))
            .execute(&mut conn)
            .await?;

        struct AggregateRow {
            id: Vec<u8>,
            version: u64,
            r#type: String,
        }
        let result = sqlx::query(include_str!("../../../sql/select_aggregates.sql"))
            .try_map(|row: SqliteRow| {
                let r#type: String = row.get("type");
                let version_as_i64: i64 = row.get("version");
                let version = u64::from_be_bytes(version_as_i64.to_be_bytes());
                let id: Vec<u8> = row.get("id"); // TODO
                Ok(AggregateRow {
                    id,
                    version,
                    r#type,
                })
            })
            .fetch_all(&mut conn)
            .await?;

        assert!(result.is_empty());
        Ok(())
    }
}
