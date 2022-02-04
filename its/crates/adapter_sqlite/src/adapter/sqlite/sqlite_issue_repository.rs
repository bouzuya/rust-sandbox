use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::Context;
use async_trait::async_trait;
use domain::{
    aggregate::{IssueAggregate, IssueAggregateEvent},
    IssueId,
};

use sqlx::{
    any::{AnyConnectOptions, AnyRow},
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    AnyPool, FromRow, Row,
};
use use_case::{IssueRepository, RepositoryError};

use crate::{
    adapter::sqlite::event_store::{AggregateVersion, Event},
    event_dto::EventDto,
};

use super::event_store::{AggregateId, EventStore};

#[derive(Debug)]
pub struct SqliteIssueRepository {
    pool: AnyPool,
}

struct IssueIdRow {
    issue_id: String,
    aggregate_id: String,
}

impl<'r> FromRow<'r, AnyRow> for IssueIdRow {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            issue_id: row.get("issue_id"),
            aggregate_id: row.get("aggregate_id"),
        })
    }
}

impl SqliteIssueRepository {
    async fn connection(path: &Path) -> anyhow::Result<AnyPool> {
        let options = SqliteConnectOptions::from_str(&format!(
            "sqlite:{}?mode=rwc",
            path.to_str().with_context(|| "invalid path")?
        ))?
        .journal_mode(SqliteJournalMode::Delete);
        let options = AnyConnectOptions::from(options);
        let pool = AnyPool::connect_with(options).await?;
        Ok(pool)
    }

    pub async fn new(path_buf: PathBuf) -> Result<Self, RepositoryError> {
        let pool = Self::connection(path_buf.as_path())
            .await
            .map_err(|_| RepositoryError::IO)?;
        let mut conn = pool.acquire().await.map_err(|_| RepositoryError::IO)?;

        // migrate
        sqlx::query(include_str!("../../../sql/create_issue_ids.sql"))
            .execute(&mut conn)
            .await
            .map_err(|_| RepositoryError::IO)?;

        Ok(Self { pool })
    }

    async fn find_aggregate_id_by_issue_id(
        &self,
        issue_id: &IssueId,
    ) -> Result<Option<AggregateId>, RepositoryError> {
        let mut tx = self.pool.begin().await.map_err(|_| RepositoryError::IO)?;

        let issue_id_row: Option<IssueIdRow> =
            sqlx::query_as(include_str!("../../../sql/select_issue_id_by_issue_id.sql"))
                .bind(issue_id.to_string())
                .fetch_optional(&mut tx)
                .await
                .map_err(|_| RepositoryError::IO)?;

        Ok(issue_id_row.map(|row| AggregateId::from_str(row.aggregate_id.as_str()).unwrap()))
    }
}

#[async_trait]
impl IssueRepository for SqliteIssueRepository {
    async fn find_by_id(
        &self,
        _issue_id: &IssueId,
    ) -> Result<Option<IssueAggregate>, RepositoryError> {
        todo!()
    }

    async fn last_created(&self) -> Result<Option<IssueAggregate>, RepositoryError> {
        todo!()
    }

    async fn save(&self, event: IssueAggregateEvent) -> Result<(), RepositoryError> {
        let path_buf = PathBuf::from("its.sqlite");
        let mut event_store = EventStore::new(path_buf)
            .await
            .map_err(|_| RepositoryError::IO)?;

        let issue_id = event.issue_id();
        let aggregate_id = self
            .find_aggregate_id_by_issue_id(issue_id)
            .await?
            .unwrap_or_else(AggregateId::generate);
        let version = event.version();
        event_store
            .save(
                None,
                Event {
                    aggregate_id,
                    data: serde_json::to_string(&EventDto::from(event))
                        .map_err(|_| RepositoryError::IO)?,
                    version: AggregateVersion::from(
                        u32::try_from(u64::from(version)).map_err(|_| RepositoryError::IO)?,
                    ),
                },
            )
            .await
            .map_err(|_| RepositoryError::IO)?;
        Ok(())
    }
}
