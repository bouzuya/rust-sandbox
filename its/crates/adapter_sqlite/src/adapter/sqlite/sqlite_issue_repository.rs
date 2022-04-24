mod issue_id_row;

use std::{num::TryFromIntError, str::FromStr};

use async_trait::async_trait;
use domain::{
    aggregate::{IssueAggregate, IssueAggregateError, IssueAggregateEvent},
    DomainEvent, IssueId, ParseDomainEventError, Version,
};

use sqlx::{any::AnyArguments, query::Query, Any, AnyPool, Transaction};
use use_case::{IssueRepository, IssueRepositoryError};

use crate::adapter::sqlite::event_store::{Event, EventStreamVersion};

use self::issue_id_row::IssueIdRow;

use super::{
    event_store::{self, EventStoreError, EventStreamId},
    rdb_connection_pool::RdbConnectionPool,
};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("EventStore")]
    EventStore(#[from] EventStoreError),
    #[error("InvalidDomainEvent")]
    InvalidDomainEvent(#[from] ParseDomainEventError),
    #[error("InvalidIssueId")]
    InvalidIssueId(TryFromIntError),
    #[error("InvalidVersion")]
    InvalidVersion(TryFromIntError),
    #[error("IssueAggregate")]
    IssueAggregate(#[from] IssueAggregateError),
    #[error("RowsAffectedNotEqualOne")]
    RowsAffectedNotEqualOne,
    #[error("Sqlx")]
    Sqlx(#[from] sqlx::Error),
    #[error("UnknownAggregateEvent")]
    UnknownAggregateEvent,
}

impl From<Error> for IssueRepositoryError {
    fn from(e: Error) -> Self {
        match e {
            Error::EventStore(e) => IssueRepositoryError::Unknown(e.to_string()),
            Error::InvalidDomainEvent(e) => IssueRepositoryError::Unknown(e.to_string()),
            Error::InvalidIssueId(e) => IssueRepositoryError::Unknown(e.to_string()),
            Error::InvalidVersion(e) => IssueRepositoryError::Unknown(e.to_string()),
            Error::IssueAggregate(e) => IssueRepositoryError::Unknown(e.to_string()),
            Error::RowsAffectedNotEqualOne => IssueRepositoryError::Unknown(e.to_string()),
            Error::Sqlx(e) => IssueRepositoryError::Unknown(e.to_string()),
            Error::UnknownAggregateEvent => IssueRepositoryError::Unknown(e.to_string()),
        }
    }
}

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

    fn events_to_issue_aggregate_events(
        events: Vec<Event>,
    ) -> Result<Vec<IssueAggregateEvent>, Error> {
        let mut aggregate_events = vec![];
        for event in events {
            let domain_event = DomainEvent::from_str(event.data.as_str())?;
            // TODO: check event.version and event_stream_id
            let aggregate_event = domain_event.issue().ok_or(Error::UnknownAggregateEvent)?;
            aggregate_events.push(aggregate_event);
        }
        Ok(aggregate_events)
    }

    async fn find_by_id(&self, issue_id: &IssueId) -> Result<Option<IssueAggregate>, Error> {
        let mut transaction = self.pool.begin().await?;
        let found = match self
            .find_event_stream_id_by_issue_id(&mut transaction, issue_id)
            .await?
        {
            None => None,
            Some(event_stream_id) => {
                let events =
                    event_store::find_events_by_event_stream_id(&mut transaction, event_stream_id)
                        .await?;
                let issue_aggregate_events = Self::events_to_issue_aggregate_events(events)?;
                IssueAggregate::from_events(&issue_aggregate_events).map(Some)?
            }
        };
        Ok(found)
    }

    async fn find_by_issue_id_and_version(
        &self,
        transaction: &mut Transaction<'_, Any>,
        issue_id: &IssueId,
        version: &Version,
    ) -> Result<Option<IssueAggregate>, Error> {
        let event_stream_id = self
            .find_event_stream_id_by_issue_id(&mut *transaction, issue_id)
            .await?;
        match event_stream_id {
            None => Ok(None),
            Some(event_stream_id) => {
                let event_stream_version = Self::version_to_event_stream_version(*version)?;
                let events =
                    event_store::find_events_by_event_stream_id_and_version_less_than_equal(
                        transaction,
                        event_stream_id,
                        event_stream_version,
                    )
                    .await?;
                let issue_aggregate_events = Self::events_to_issue_aggregate_events(events)?;
                let issue = IssueAggregate::from_events(&issue_aggregate_events)?;
                if issue.version() != *version {
                    Ok(None)
                } else {
                    Ok(Some(issue))
                }
            }
        }
    }

    async fn find_event_stream_id_by_issue_id(
        &self,
        transaction: &mut Transaction<'_, Any>,
        issue_id: &IssueId,
    ) -> Result<Option<EventStreamId>, Error> {
        let issue_id_row: Option<IssueIdRow> = sqlx::query_as(include_str!(
            "../../../sql/command/select_issue_id_by_issue_id.sql"
        ))
        .bind(i64::try_from(usize::from(issue_id.issue_number())).map_err(Error::InvalidIssueId)?)
        .fetch_optional(&mut *transaction)
        .await?;
        Ok(issue_id_row.map(|row| row.event_stream_id()))
    }

    async fn find_max_issue_id(
        &self,
        transaction: &mut Transaction<'_, Any>,
    ) -> Result<Option<IssueId>, Error> {
        let issue_id_row: Option<IssueIdRow> =
            sqlx::query_as(include_str!("../../../sql/command/select_max_issue_id.sql"))
                .fetch_optional(transaction)
                .await?;
        Ok(issue_id_row.map(|row| row.issue_id()))
    }

    async fn insert_issue_id(
        &self,
        transaction: &mut Transaction<'_, Any>,
        issue_id: &IssueId,
        event_stream_id: EventStreamId,
    ) -> Result<(), Error> {
        let query: Query<Any, AnyArguments> =
            sqlx::query(include_str!("../../../sql/command/insert_issue_id.sql"))
                .bind(Self::issue_number_as_i64_from_issue_id(issue_id)?)
                .bind(event_stream_id.to_string());
        let rows_affected = query.execute(transaction).await?.rows_affected();
        if rows_affected != 1 {
            return Err(Error::RowsAffectedNotEqualOne);
        }

        Ok(())
    }

    fn issue_number_as_i64_from_issue_id(issue_id: &IssueId) -> Result<i64, Error> {
        i64::try_from(usize::from(issue_id.issue_number())).map_err(Error::InvalidIssueId)
    }

    async fn last_created(&self) -> Result<Option<IssueAggregate>, Error> {
        let mut transaction = self.pool.begin().await?;
        Ok(match self.find_max_issue_id(&mut transaction).await? {
            Some(issue_id) => self.find_by_id(&issue_id).await?,
            None => None,
        })
    }

    async fn save(&self, issue: &IssueAggregate) -> Result<(), Error> {
        let mut transaction = self.pool.begin().await?;
        for event in issue.events().iter().cloned() {
            let issue_id = event.issue_id().clone();
            if let Some(event_stream_id) = self
                .find_event_stream_id_by_issue_id(&mut transaction, &issue_id)
                .await?
            {
                // update
                let version = event.version();
                event_store::save(
                    &mut transaction,
                    version
                        .prev()
                        .map(Self::version_to_event_stream_version)
                        .transpose()?,
                    Event {
                        event_stream_id,
                        data: DomainEvent::from(event).to_string(),
                        version: Self::version_to_event_stream_version(version)?,
                    },
                )
                .await?
            } else {
                // create
                let event_stream_id = EventStreamId::generate();
                let version = event.version();
                event_store::save(
                    &mut transaction,
                    None,
                    Event {
                        event_stream_id,
                        data: DomainEvent::from(event).to_string(),
                        version: Self::version_to_event_stream_version(version)?,
                    },
                )
                .await?;
                self.insert_issue_id(&mut transaction, &issue_id, event_stream_id)
                    .await?;
            }
        }

        transaction.commit().await?;
        Ok(())
    }

    fn version_to_event_stream_version(version: Version) -> Result<EventStreamVersion, Error> {
        u32::try_from(u64::from(version))
            .map(EventStreamVersion::from)
            .map_err(Error::InvalidVersion)
    }
}

#[async_trait]
impl IssueRepository for SqliteIssueRepository {
    async fn find_by_id(
        &self,
        issue_id: &IssueId,
    ) -> Result<Option<IssueAggregate>, IssueRepositoryError> {
        Ok(Self::find_by_id(self, issue_id).await?)
    }

    async fn last_created(&self) -> Result<Option<IssueAggregate>, IssueRepositoryError> {
        Ok(Self::last_created(self).await?)
    }

    async fn save(&self, issue: &IssueAggregate) -> Result<(), IssueRepositoryError> {
        Ok(Self::save(self, issue).await?)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use anyhow::Context;
    use domain::IssueResolution;
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
        let found = found.ok_or_else(|| anyhow::anyhow!("found is not Some"))?;

        // update
        let resolution = IssueResolution::from_str("Duplicate")?;
        let updated = found.finish(Some(resolution), Instant::now())?;
        issue_repository.save(&updated).await?;

        let found = issue_repository.find_by_id(updated.id()).await?;
        assert_eq!(Some(updated.truncate_events()), found);

        Ok(())
    }
}
