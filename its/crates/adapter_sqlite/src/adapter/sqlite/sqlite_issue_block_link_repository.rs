use std::str::FromStr;

use async_trait::async_trait;
use domain::{
    aggregate::{IssueBlockLinkAggregate, IssueBlockLinkAggregateEvent},
    DomainEvent, IssueBlockLinkId,
};

use sqlx::{any::AnyRow, Any, AnyPool, FromRow, Row, Transaction};
use use_case::{IssueBlockLinkRepository, IssueBlockLinkRepositoryError};

use crate::{event_dto::EventDto, SqliteConnectionPool};

use super::event_store::{self, AggregateId};

#[derive(Debug)]
struct IssueBlockLinkIdRow {
    issue_block_link_id: String,
    aggregate_id: String,
}

impl IssueBlockLinkIdRow {
    fn issue_block_link_id(&self) -> IssueBlockLinkId {
        IssueBlockLinkId::from_str(&self.issue_block_link_id)
            .expect("stored issue_block_link_id is not well-formed")
    }

    fn aggregate_id(&self) -> AggregateId {
        AggregateId::from_str(&self.aggregate_id)
            .expect("stored issue_block_link_id is not well-formed")
    }
}

impl<'r> FromRow<'r, AnyRow> for IssueBlockLinkIdRow {
    fn from_row(row: &'r AnyRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            issue_block_link_id: row.get("issue_block_link_id"),
            aggregate_id: row.get("aggregate_id"),
        })
    }
}

#[derive(Debug)]
pub struct SqliteIssueBlockLinkRepository {
    pool: AnyPool,
}

impl SqliteIssueBlockLinkRepository {
    pub async fn new(
        connection_pool: SqliteConnectionPool,
    ) -> Result<Self, IssueBlockLinkRepositoryError> {
        Ok(Self {
            pool: AnyPool::from(connection_pool),
        })
    }

    async fn find_aggregate_id_by_issue_block_link_id(
        &self,
        transaction: &mut Transaction<'_, Any>,
        issue_block_link_id: &IssueBlockLinkId,
    ) -> Result<Option<AggregateId>, IssueBlockLinkRepositoryError> {
        let issue_block_link_id_row: Option<IssueBlockLinkIdRow> = sqlx::query_as(include_str!(
            "../../../sql/command/select_issue_block_link_id_by_issue_block_link_id.sql"
        ))
        .bind(issue_block_link_id.to_string())
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|e| IssueBlockLinkRepositoryError::Unknown(e.to_string()))?;
        Ok(issue_block_link_id_row.map(|row| row.aggregate_id()))
    }
}

#[async_trait]
impl IssueBlockLinkRepository for SqliteIssueBlockLinkRepository {
    async fn find_by_id(
        &self,
        issue_block_link_id: &IssueBlockLinkId,
    ) -> Result<Option<IssueBlockLinkAggregate>, IssueBlockLinkRepositoryError> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|e| IssueBlockLinkRepositoryError::Unknown(e.to_string()))?;
        match self
            .find_aggregate_id_by_issue_block_link_id(&mut transaction, issue_block_link_id)
            .await?
        {
            Some(aggregate_id) => {
                let events =
                    event_store::find_events_by_aggregate_id(&mut transaction, aggregate_id)
                        .await
                        .map_err(|e| IssueBlockLinkRepositoryError::Unknown(e.to_string()))?;
                let mut issue_block_link_aggregate_events = vec![];
                for event in events {
                    let dto = serde_json::from_str::<'_, EventDto>(event.data.as_str())
                        .map_err(|e| IssueBlockLinkRepositoryError::Unknown(e.to_string()))?;
                    // TODO: check dto.version and aggregate_id
                    issue_block_link_aggregate_events.push(
                        DomainEvent::try_from(dto)
                            .map_err(|e| IssueBlockLinkRepositoryError::Unknown(e.to_string()))?
                            .issue_block_link()
                            .ok_or_else(|| {
                                IssueBlockLinkRepositoryError::Unknown("".to_string())
                            })?,
                    );
                }
                IssueBlockLinkAggregate::from_events(&issue_block_link_aggregate_events)
                    .map(Some)
                    .map_err(|e| IssueBlockLinkRepositoryError::Unknown(e.to_string()))
            }
            None => Ok(None),
        }
    }

    async fn save(
        &self,
        _event: IssueBlockLinkAggregateEvent,
    ) -> Result<(), IssueBlockLinkRepositoryError> {
        todo!()
    }
}
