mod issue_comment_id_row;

use std::{num::TryFromIntError, str::FromStr};

use async_trait::async_trait;
use domain::{
    aggregate::issue_comment::IssueCommentAggregate, DomainEvent, IssueCommentId,
    ParseDomainEventError, Version,
};

use event_store::{Event, EventId, EventStreamId, EventStreamSeq};
use sqlx::{any::AnyArguments, query::Query, Any, AnyPool, Transaction};
use use_case::issue_comment_repository::IssueCommentRepository;

use self::issue_comment_id_row::IssueCommentIdRow;

use super::rdb_connection_pool::RdbConnectionPool;

type Aggregate = IssueCommentAggregate;
type AggregateError = domain::aggregate::issue_comment::Error;
type AggregateEvent = domain::aggregate::issue_comment::Event;
type AggregateId = IssueCommentId;

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
    #[error("IssueCommentAggregate")]
    IssueCommentAggregate(#[from] AggregateError),
    #[error("RowsAffectedNotEqualOne")]
    RowsAffectedNotEqualOne,
    #[error("Sqlx")]
    Sqlx(#[from] sqlx::Error),
    #[error("UnknownAggregateEvent")]
    UnknownAggregateEvent,
}

type Result<T, E = Error> = std::result::Result<T, E>;

impl From<Error> for use_case::issue_comment_repository::Error {
    fn from(e: Error) -> Self {
        use use_case::issue_comment_repository::Error as E;
        match e {
            Error::EventStore(e) => E::Unknown(e.to_string()),
            Error::InvalidDomainEvent(e) => E::Unknown(e.to_string()),
            Error::InvalidIssueId(e) => E::Unknown(e.to_string()),
            Error::InvalidVersion(e) => E::Unknown(e.to_string()),
            Error::IssueCommentAggregate(e) => E::Unknown(e.to_string()),
            Error::RowsAffectedNotEqualOne => E::Unknown(e.to_string()),
            Error::Sqlx(e) => E::Unknown(e.to_string()),
            Error::UnknownAggregateEvent => E::Unknown(e.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct SqliteIssueCommentRepository {
    pool: AnyPool,
}

impl SqliteIssueCommentRepository {
    pub(super) fn new(connection_pool: RdbConnectionPool) -> Result<Self> {
        Ok(Self {
            pool: AnyPool::from(connection_pool),
        })
    }

    fn events_to_aggregate_events(events: Vec<Event>) -> Result<Vec<AggregateEvent>> {
        let mut aggregate_events = vec![];
        for event in events {
            let domain_event = DomainEvent::from_str(event.data.as_str())?;
            // TODO: check event.version and event_stream_id
            let aggregate_event = domain_event
                .issue_comment()
                .ok_or(Error::UnknownAggregateEvent)?;
            aggregate_events.push(aggregate_event);
        }
        Ok(aggregate_events)
    }

    async fn find_by_id(&self, aggregate_id: &AggregateId) -> Result<Option<Aggregate>> {
        let mut transaction = self.pool.begin().await?;
        let found = match self
            .find_event_stream_id_by_aggregate_id(&mut transaction, aggregate_id)
            .await?
        {
            None => None,
            Some(event_stream_id) => {
                let events =
                    event_store::find_events_by_event_stream_id(&mut transaction, event_stream_id)
                        .await?;
                let aggregate_events = Self::events_to_aggregate_events(events)?;
                Aggregate::from_events(&aggregate_events)?
            }
        };
        Ok(found)
    }

    async fn find_by_id_and_version(
        &self,
        aggregate_id: &AggregateId,
        version: &Version,
    ) -> Result<Option<Aggregate>> {
        let mut transaction = self.pool.begin().await?;
        let event_stream_id = self
            .find_event_stream_id_by_aggregate_id(&mut transaction, aggregate_id)
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
                let aggregate_events = Self::events_to_aggregate_events(events)?;
                Ok(Aggregate::from_events(&aggregate_events)?
                    .and_then(|a| (a.version() == *version).then_some(a)))
            }
        }
    }

    async fn find_event_stream_id_by_aggregate_id(
        &self,
        transaction: &mut Transaction<'_, Any>,
        aggregate_id: &AggregateId,
    ) -> Result<Option<EventStreamId>> {
        let issue_comment_id_row: Option<IssueCommentIdRow> = sqlx::query_as(include_str!(
            "../../../sql/command/select_issue_comment_id_by_issue_comment_id.sql"
        ))
        .bind(aggregate_id.to_string())
        .fetch_optional(&mut *transaction)
        .await?;
        Ok(issue_comment_id_row.map(|row| row.event_stream_id()))
    }

    async fn insert_aggregate_id(
        &self,
        transaction: &mut Transaction<'_, Any>,
        aggregate_id: &AggregateId,
        event_stream_id: EventStreamId,
    ) -> Result<()> {
        let query: Query<Any, AnyArguments> = sqlx::query(include_str!(
            "../../../sql/command/insert_issue_comment_id.sql"
        ))
        .bind(aggregate_id.to_string())
        .bind(event_stream_id.to_string());
        let rows_affected = query.execute(transaction).await?.rows_affected();
        if rows_affected != 1 {
            return Err(Error::RowsAffectedNotEqualOne);
        }

        Ok(())
    }

    async fn save(&self, aggregate: &Aggregate) -> Result<()> {
        let mut transaction = self.pool.begin().await?;
        for event in aggregate.events().iter().cloned() {
            let aggregate_id = event.issue_comment_id().clone();
            if let Some(event_stream_id) = self
                .find_event_stream_id_by_aggregate_id(&mut transaction, &aggregate_id)
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
                self.insert_aggregate_id(&mut transaction, &aggregate_id, event_stream_id)
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
impl IssueCommentRepository for SqliteIssueCommentRepository {
    async fn find_by_id(
        &self,
        issue_comment_id: &IssueCommentId,
    ) -> use_case::issue_comment_repository::Result<Option<Aggregate>> {
        Ok(Self::find_by_id(self, issue_comment_id).await?)
    }

    async fn find_by_id_and_version(
        &self,
        issue_comment_id: &IssueCommentId,
        version: &Version,
    ) -> use_case::issue_comment_repository::Result<Option<Aggregate>> {
        Ok(Self::find_by_id_and_version(self, issue_comment_id, version).await?)
    }

    async fn save(
        &self,
        issue_comment: &IssueCommentAggregate,
    ) -> use_case::issue_comment_repository::Result<()> {
        Ok(Self::save(self, issue_comment).await?)
    }
}

#[cfg(test)]
mod tests {
    use domain::IssueCommentId;
    use limited_date_time::Instant;

    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let connection_pool = RdbConnectionPool::new("sqlite::memory:").await?;
        let issue_comment_repository = connection_pool.issue_comment_repository()?;

        // create
        let created = Aggregate::new(
            Instant::now(),
            IssueCommentId::generate(),
            "123".parse()?,
            "text".parse()?,
        )?;
        issue_comment_repository.save(&created).await?;
        let found = issue_comment_repository.find_by_id(created.id()).await?;
        assert_eq!(Some(created.truncate_events()), found);
        let found = found.ok_or_else(|| anyhow::anyhow!("found is not Some"))?;

        // update
        let updated = found.update("text".parse()?, Instant::now())?;
        issue_comment_repository.save(&updated).await?;
        let found = issue_comment_repository.find_by_id(updated.id()).await?;
        assert_eq!(Some(updated.truncate_events()), found);
        let found = found.ok_or_else(|| anyhow::anyhow!("found is not Some"))?;

        // delete
        let deleted = found.delete(Instant::now())?;
        issue_comment_repository.save(&deleted).await?;
        let found = issue_comment_repository.find_by_id(deleted.id()).await?;
        assert_eq!(None, found);

        Ok(())
    }
}
