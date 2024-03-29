mod issue_id_row;

use std::{num::TryFromIntError, str::FromStr};

use async_trait::async_trait;
use domain::{
    aggregate::{IssueAggregate, IssueAggregateEvent},
    DomainEvent, IssueId, ParseDomainEventError, Version,
};

use event_store::{Event, EventId, EventStreamId, EventStreamSeq};
use sqlx::{any::AnyArguments, query::Query, Any, AnyPool, Transaction};
use use_case::IssueRepository;

use self::issue_id_row::IssueIdRow;

use super::rdb_connection_pool::RdbConnectionPool;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("EventStore")]
    EventStore(#[from] event_store::Error),
    #[error("InvalidDomainEvent")]
    InvalidDomainEvent(#[from] ParseDomainEventError),
    #[error("InvalidIssueId")]
    InvalidIssueId(TryFromIntError),
    #[error("InvalidVersion")]
    InvalidVersion(TryFromIntError),
    #[error("IssueAggregate")]
    IssueAggregate(#[from] domain::aggregate::issue::Error),
    #[error("RowsAffectedNotEqualOne")]
    RowsAffectedNotEqualOne,
    #[error("Sqlx")]
    Sqlx(#[from] sqlx::Error),
    #[error("UnknownAggregateEvent")]
    UnknownAggregateEvent,
}

type Result<T, E = Error> = std::result::Result<T, E>;

impl From<Error> for use_case::issue_repository::Error {
    fn from(e: Error) -> Self {
        use use_case::issue_repository::Error as E;
        match e {
            Error::EventStore(e) => E::Unknown(e.to_string()),
            Error::InvalidDomainEvent(e) => E::Unknown(e.to_string()),
            Error::InvalidIssueId(e) => E::Unknown(e.to_string()),
            Error::InvalidVersion(e) => E::Unknown(e.to_string()),
            Error::IssueAggregate(e) => E::Unknown(e.to_string()),
            Error::RowsAffectedNotEqualOne => E::Unknown(e.to_string()),
            Error::Sqlx(e) => E::Unknown(e.to_string()),
            Error::UnknownAggregateEvent => E::Unknown(e.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct SqliteIssueRepository {
    pool: AnyPool,
}

impl SqliteIssueRepository {
    pub(super) fn new(connection_pool: RdbConnectionPool) -> Result<Self> {
        Ok(Self {
            pool: AnyPool::from(connection_pool),
        })
    }

    fn events_to_issue_aggregate_events(events: Vec<Event>) -> Result<Vec<IssueAggregateEvent>> {
        let mut aggregate_events = vec![];
        for event in events {
            let domain_event = DomainEvent::from_str(event.data.as_str())?;
            // TODO: check event.version and event_stream_id
            let aggregate_event = domain_event.issue().ok_or(Error::UnknownAggregateEvent)?;
            aggregate_events.push(aggregate_event);
        }
        Ok(aggregate_events)
    }

    async fn find_by_id(&self, issue_id: &IssueId) -> Result<Option<IssueAggregate>> {
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

    async fn find_by_id_and_version(
        &self,
        issue_id: &IssueId,
        version: &Version,
    ) -> Result<Option<IssueAggregate>> {
        let mut transaction = self.pool.begin().await?;
        let event_stream_id = self
            .find_event_stream_id_by_issue_id(&mut transaction, issue_id)
            .await?;
        match event_stream_id {
            None => Ok(None),
            Some(event_stream_id) => {
                let event_stream_version = Self::version_to_event_stream_version(*version)?;
                let events =
                    event_store::find_events_by_event_stream_id_and_version_less_than_equal(
                        &mut transaction,
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
    ) -> Result<Option<EventStreamId>> {
        let issue_id_row: Option<IssueIdRow> = sqlx::query_as(include_str!(
            "../../../sql/command/select_issue_id_by_issue_id.sql"
        ))
        .bind(Self::issue_number_as_i64_from_issue_id(issue_id)?)
        .fetch_optional(&mut *transaction)
        .await?;
        Ok(issue_id_row.map(|row| row.event_stream_id()))
    }

    async fn find_max_issue_id(
        &self,
        transaction: &mut Transaction<'_, Any>,
    ) -> Result<Option<IssueId>> {
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
    ) -> Result<()> {
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

    fn issue_number_as_i64_from_issue_id(issue_id: &IssueId) -> Result<i64> {
        i64::try_from(usize::from(issue_id.issue_number())).map_err(Error::InvalidIssueId)
    }

    async fn last_created(&self) -> Result<Option<IssueAggregate>> {
        let mut transaction = self.pool.begin().await?;
        Ok(match self.find_max_issue_id(&mut transaction).await? {
            Some(issue_id) => self.find_by_id(&issue_id).await?,
            None => None,
        })
    }

    async fn save(&self, issue: &IssueAggregate) -> Result<()> {
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
                        id: EventId::generate(),
                        stream_id: event_stream_id,
                        stream_seq: Self::version_to_event_stream_version(version)?,
                        data: DomainEvent::from(event).to_string(),
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
                        id: EventId::generate(),
                        stream_id: event_stream_id,
                        data: DomainEvent::from(event).to_string(),
                        stream_seq: Self::version_to_event_stream_version(version)?,
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

    fn version_to_event_stream_version(version: Version) -> Result<EventStreamSeq> {
        u32::try_from(u64::from(version))
            .map(EventStreamSeq::from)
            .map_err(Error::InvalidVersion)
    }
}

#[async_trait]
impl IssueRepository for SqliteIssueRepository {
    async fn find_by_id(
        &self,
        issue_id: &IssueId,
    ) -> use_case::issue_repository::Result<Option<IssueAggregate>> {
        Ok(Self::find_by_id(self, issue_id).await?)
    }

    async fn find_by_id_and_version(
        &self,
        issue_id: &IssueId,
        version: &Version,
    ) -> use_case::issue_repository::Result<Option<IssueAggregate>> {
        Ok(Self::find_by_id_and_version(self, issue_id, version).await?)
    }

    async fn last_created(&self) -> use_case::issue_repository::Result<Option<IssueAggregate>> {
        Ok(Self::last_created(self).await?)
    }

    async fn save(&self, issue: &IssueAggregate) -> use_case::issue_repository::Result<()> {
        Ok(Self::save(self, issue).await?)
    }
}

#[cfg(test)]
mod tests {
    use domain::aggregate::issue::IssueResolution;
    use limited_date_time::Instant;

    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let connection_pool = RdbConnectionPool::new("sqlite::memory:").await?;
        let issue_repository = connection_pool.issue_repository()?;

        // create
        let created = IssueAggregate::new(
            Instant::now(),
            "123".parse()?,
            "title".parse()?,
            Some("2021-02-03T04:05:06Z".parse()?),
            "desc1".parse()?,
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
