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
    any::{AnyArguments, AnyConnectOptions, AnyRow},
    query::Query,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    Any, AnyPool, FromRow, Row, Transaction,
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
        transaction: &mut Transaction<'_, Any>,
        issue_id: &IssueId,
    ) -> Result<Option<AggregateId>, RepositoryError> {
        let issue_id_row: Option<IssueIdRow> =
            sqlx::query_as(include_str!("../../../sql/select_issue_id_by_issue_id.sql"))
                .bind(issue_id.to_string())
                .fetch_optional(transaction)
                .await
                .map_err(|_| RepositoryError::IO)?;

        Ok(issue_id_row.map(|row| AggregateId::from_str(row.aggregate_id.as_str()).unwrap()))
    }

    async fn insert_issue_id(
        &self,
        transaction: &mut Transaction<'_, Any>,
        issue_id: &IssueId,
        aggregate_id: AggregateId,
    ) -> Result<(), RepositoryError> {
        let query: Query<Any, AnyArguments> =
            sqlx::query(include_str!("../../../sql/insert_issue_id.sql"))
                .bind(issue_id.to_string())
                .bind(aggregate_id.to_string());
        let result = query
            .execute(transaction)
            .await
            .map_err(|_| RepositoryError::IO)?;
        if result.rows_affected() != 1 {
            return Err(RepositoryError::IO);
        }

        Ok(())
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
        let mut transaction = self.pool.begin().await.map_err(|_| RepositoryError::IO)?;

        let issue_id = event.issue_id();
        if let Some(aggregate_id) = self
            .find_aggregate_id_by_issue_id(&mut transaction, issue_id)
            .await?
        {
            // update
            let version = event.version();
            EventStore::save(
                &mut transaction,
                None, // FIXME
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
        } else {
            // create
            let aggregate_id = AggregateId::generate();
            self.insert_issue_id(&mut transaction, issue_id, aggregate_id)
                .await?;

            let version = event.version();
            EventStore::save(
                &mut transaction,
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
        }

        transaction
            .commit()
            .await
            .map_err(|_| RepositoryError::IO)?;
        Ok(())
    }
}
