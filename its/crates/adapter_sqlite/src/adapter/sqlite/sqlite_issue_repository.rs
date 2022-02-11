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

use super::event_store::{self, AggregateId};

#[derive(Debug)]
pub struct SqliteIssueRepository {
    pool: AnyPool,
}

struct IssueIdRow {
    issue_id: String,
    aggregate_id: String,
}

impl IssueIdRow {
    fn issue_id(&self) -> IssueId {
        IssueId::from_str(self.issue_id.as_str()).unwrap()
    }

    fn aggregate_id(&self) -> AggregateId {
        AggregateId::from_str(self.aggregate_id.as_str()).unwrap()
    }
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

        Ok(issue_id_row.map(|row| row.aggregate_id()))
    }

    async fn find_max_issue_id(
        &self,
        transaction: &mut Transaction<'_, Any>,
    ) -> Result<Option<IssueId>, RepositoryError> {
        let issue_id_row: Option<IssueIdRow> =
            sqlx::query_as(include_str!("../../../sql/select_max_issue_id.sql"))
                .fetch_optional(transaction)
                .await
                .map_err(|_| RepositoryError::IO)?;
        Ok(issue_id_row.map(|row| row.issue_id()))
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
        issue_id: &IssueId,
    ) -> Result<Option<IssueAggregate>, RepositoryError> {
        let mut transaction = self.pool.begin().await.map_err(|_| RepositoryError::IO)?;
        match self
            .find_aggregate_id_by_issue_id(&mut transaction, issue_id)
            .await?
        {
            Some(aggregate_id) => {
                let events =
                    event_store::find_events_by_aggregate_id(&mut transaction, aggregate_id)
                        .await
                        .map_err(|_| RepositoryError::IO)?;
                let mut issue_aggregate_events = vec![];
                for event in events {
                    let dto = serde_json::from_str::<'_, EventDto>(event.data.as_str())
                        .map_err(|_| RepositoryError::IO)?;
                    // TODO: check dto.version and aggregate_id
                    issue_aggregate_events
                        .push(IssueAggregateEvent::try_from(dto).map_err(|_| RepositoryError::IO)?);
                }
                IssueAggregate::from_events(&issue_aggregate_events)
                    .map(Some)
                    .map_err(|_| RepositoryError::IO)
            }
            None => Ok(None),
        }
    }

    async fn last_created(&self) -> Result<Option<IssueAggregate>, RepositoryError> {
        let mut transaction = self.pool.begin().await.map_err(|_| RepositoryError::IO)?;
        Ok(match self.find_max_issue_id(&mut transaction).await? {
            Some(issue_id) => self.find_by_id(&issue_id).await?,
            None => None,
        })
    }

    async fn save(&self, event: IssueAggregateEvent) -> Result<(), RepositoryError> {
        let mut transaction = self.pool.begin().await.map_err(|_| RepositoryError::IO)?;

        let issue_id = event.issue_id().clone();
        if let Some(aggregate_id) = self
            .find_aggregate_id_by_issue_id(&mut transaction, &issue_id)
            .await?
        {
            // update
            let version = event.version();
            event_store::save(
                &mut transaction,
                version.prev().map(|version| {
                    u32::try_from(u64::from(version))
                        .map(AggregateVersion::from)
                        .map_err(|_| RepositoryError::IO)
                        .expect("parse aggregate version")
                }),
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
            let version = event.version();
            event_store::save(
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

            self.insert_issue_id(&mut transaction, &issue_id, aggregate_id)
                .await?;
        }

        transaction
            .commit()
            .await
            .map_err(|_| RepositoryError::IO)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use domain::aggregate::{
        IssueAggregateCommand, IssueAggregateCreateIssue, IssueAggregateFinishIssue,
    };
    use limited_date_time::Instant;
    use tempfile::tempdir;

    use super::*;

    // TODO:
    async fn migrate(path: &Path) -> anyhow::Result<()> {
        let options = SqliteConnectOptions::from_str(&format!(
            "sqlite:{}?mode=rwc",
            path.to_str().with_context(|| "invalid path")?
        ))?
        .journal_mode(SqliteJournalMode::Delete);
        let options = AnyConnectOptions::from(options);
        let pool = AnyPool::connect_with(options).await?;
        let mut transaction = pool.begin().await?;
        event_store::migrate(&mut transaction).await?;
        transaction.commit().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;

        let sqlite_path = temp_dir.path().join("its.sqlite");
        migrate(sqlite_path.as_path()).await?;

        let issue_repository = SqliteIssueRepository::new(sqlite_path).await?;

        // create
        let (created, created_event) = IssueAggregate::transaction(IssueAggregateCommand::Create(
            IssueAggregateCreateIssue {
                issue_number: "123".parse()?,
                issue_title: "title".parse()?,
                issue_due: Some("2021-02-03T04:05:06Z".parse()?),
                at: Instant::now(),
            },
        ))?;
        issue_repository.save(created_event).await?;

        // last_created
        let last_created = issue_repository.last_created().await?;
        assert_eq!(Some(created.clone()), last_created);

        // find_by_id
        let found = issue_repository.find_by_id(created.id()).await?;
        assert_eq!(Some(created), found);
        let found = found.ok_or(anyhow::anyhow!("found is not Some"))?;

        // update
        let (updated, updated_event) = IssueAggregate::transaction(IssueAggregateCommand::Finish(
            IssueAggregateFinishIssue {
                issue: found,
                at: Instant::now(),
            },
        ))?;
        issue_repository.save(updated_event).await?;

        let found = issue_repository.find_by_id(updated.id()).await?;
        assert_eq!(Some(updated), found);

        Ok(())
    }
}
