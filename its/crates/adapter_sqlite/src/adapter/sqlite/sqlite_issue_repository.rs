mod issue_row;

use std::str::FromStr;

use async_trait::async_trait;
use domain::{aggregate::IssueAggregate, DomainEvent, IssueId};

use sqlx::{any::AnyArguments, query::Query, Any, AnyPool, Transaction};
use use_case::{IssueRepository, IssueRepositoryError};

use crate::adapter::sqlite::event_store::{AggregateVersion, Event};

use self::issue_row::IssueIdRow;

use super::{
    event_store::{self, AggregateId},
    rdb_connection_pool::RdbConnectionPool,
};

#[derive(Debug)]
pub struct SqliteIssueRepository {
    pool: AnyPool,
}

impl SqliteIssueRepository {
    pub async fn new(connection_pool: RdbConnectionPool) -> Result<Self, IssueRepositoryError> {
        Ok(Self {
            pool: AnyPool::from(connection_pool),
        })
    }

    async fn find_aggregate_id_by_issue_id(
        &self,
        transaction: &mut Transaction<'_, Any>,
        issue_id: &IssueId,
    ) -> Result<Option<AggregateId>, IssueRepositoryError> {
        let issue_id_row: Option<IssueIdRow> = sqlx::query_as(include_str!(
            "../../../sql/command/select_issue_id_by_issue_id.sql"
        ))
        .bind(
            i64::try_from(usize::from(issue_id.issue_number())).map_err(|_| {
                IssueRepositoryError::Unknown("Failed to convert issue_number to i64".to_string())
            })?,
        )
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|e| IssueRepositoryError::Unknown(e.to_string()))?;

        Ok(issue_id_row.map(|row| row.aggregate_id()))
    }

    async fn find_max_issue_id(
        &self,
        transaction: &mut Transaction<'_, Any>,
    ) -> Result<Option<IssueId>, IssueRepositoryError> {
        let issue_id_row: Option<IssueIdRow> =
            sqlx::query_as(include_str!("../../../sql/command/select_max_issue_id.sql"))
                .fetch_optional(transaction)
                .await
                .map_err(|_| IssueRepositoryError::IO)?;
        Ok(issue_id_row.map(|row| row.issue_id()))
    }

    async fn insert_issue_id(
        &self,
        transaction: &mut Transaction<'_, Any>,
        issue_id: &IssueId,
        aggregate_id: AggregateId,
    ) -> Result<(), IssueRepositoryError> {
        let query: Query<Any, AnyArguments> =
            sqlx::query(include_str!("../../../sql/command/insert_issue_id.sql"))
                .bind(
                    i64::try_from(usize::from(issue_id.issue_number())).map_err(|_| {
                        IssueRepositoryError::Unknown(
                            "Failed to convert issue_number to i64".to_string(),
                        )
                    })?,
                )
                .bind(aggregate_id.to_string());
        let rows_affected = query
            .execute(transaction)
            .await
            .map_err(|_| IssueRepositoryError::IO)?
            .rows_affected();
        if rows_affected != 1 {
            return Err(IssueRepositoryError::IO);
        }

        Ok(())
    }
}

#[async_trait]
impl IssueRepository for SqliteIssueRepository {
    async fn find_by_id(
        &self,
        issue_id: &IssueId,
    ) -> Result<Option<IssueAggregate>, IssueRepositoryError> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|_| IssueRepositoryError::IO)?;
        match self
            .find_aggregate_id_by_issue_id(&mut transaction, issue_id)
            .await?
        {
            Some(aggregate_id) => {
                let events =
                    event_store::find_events_by_aggregate_id(&mut transaction, aggregate_id)
                        .await
                        .map_err(|_| IssueRepositoryError::IO)?;
                let mut issue_aggregate_events = vec![];
                for event in events {
                    let event = DomainEvent::from_str(event.data.as_str())
                        .map_err(|_| IssueRepositoryError::IO)?;
                    // TODO: check event.version and aggregate_id
                    issue_aggregate_events.push(event.issue().ok_or(IssueRepositoryError::IO)?);
                }
                IssueAggregate::from_events(&issue_aggregate_events)
                    .map(Some)
                    .map_err(|_| IssueRepositoryError::IO)
            }
            None => Ok(None),
        }
    }

    async fn last_created(&self) -> Result<Option<IssueAggregate>, IssueRepositoryError> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|_| IssueRepositoryError::IO)?;
        Ok(match self.find_max_issue_id(&mut transaction).await? {
            Some(issue_id) => self.find_by_id(&issue_id).await?,
            None => None,
        })
    }

    async fn save(&self, issue: &IssueAggregate) -> Result<(), IssueRepositoryError> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|_| IssueRepositoryError::IO)?;

        for event in issue.events().iter().cloned() {
            let issue_id = event.issue_id().clone();
            if let Some(aggregate_id) = self
                .find_aggregate_id_by_issue_id(&mut transaction, &issue_id)
                .await?
            {
                // update
                let version = event.version();
                event_store::save(
                    &mut transaction,
                    version
                        .prev()
                        .map(|version| {
                            u32::try_from(u64::from(version))
                                .map(AggregateVersion::from)
                                .map_err(|_| IssueRepositoryError::IO)
                        })
                        .transpose()?,
                    Event {
                        aggregate_id,
                        data: DomainEvent::from(event).to_string(),
                        version: AggregateVersion::from(
                            u32::try_from(u64::from(version))
                                .map_err(|_| IssueRepositoryError::IO)?,
                        ),
                    },
                )
                .await
                .map_err(|_| IssueRepositoryError::IO)?;
            } else {
                // create
                let aggregate_id = AggregateId::generate();
                let version = event.version();
                event_store::save(
                    &mut transaction,
                    None,
                    Event {
                        aggregate_id,
                        data: DomainEvent::from(event).to_string(),
                        version: AggregateVersion::from(
                            u32::try_from(u64::from(version))
                                .map_err(|_| IssueRepositoryError::IO)?,
                        ),
                    },
                )
                .await
                .map_err(|_| IssueRepositoryError::IO)?;

                self.insert_issue_id(&mut transaction, &issue_id, aggregate_id)
                    .await?;
            }
        }

        transaction
            .commit()
            .await
            .map_err(|_| IssueRepositoryError::IO)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use anyhow::Context;
    use limited_date_time::Instant;
    use tempfile::tempdir;

    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;

        let sqlite_dir = temp_dir.path().join("its");
        let data_dir = sqlite_dir;
        if !data_dir.exists() {
            fs::create_dir_all(data_dir.as_path())?;
        }
        let path = data_dir.join("command.sqlite");
        let connection_uri = format!(
            "sqlite:{}?mode=rwc",
            path.to_str().context("path is not utf-8")?
        );
        let connection_pool = RdbConnectionPool::new(&connection_uri).await?;
        let issue_repository = SqliteIssueRepository::new(connection_pool).await?;

        // create
        let created = IssueAggregate::new(
            Instant::now(),
            "123".parse()?,
            "title".parse()?,
            Some("2021-02-03T04:05:06Z".parse()?),
        )?;
        issue_repository.save(&created).await?;

        // last_created
        let last_created = issue_repository.last_created().await?;
        assert_eq!(Some(created.clone().truncate_events()), last_created);

        // find_by_id
        let found = issue_repository.find_by_id(created.id()).await?;
        assert_eq!(Some(created.truncate_events()), found);
        let found = found.ok_or(anyhow::anyhow!("found is not Some"))?;

        // update
        let updated = found.finish(Instant::now())?;
        issue_repository.save(&updated).await?;

        let found = issue_repository.find_by_id(updated.id()).await?;
        assert_eq!(Some(updated.truncate_events()), found);

        Ok(())
    }
}
