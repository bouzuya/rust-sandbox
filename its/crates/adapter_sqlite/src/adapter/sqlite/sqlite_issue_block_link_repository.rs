mod issue_block_link_id_row;

use std::str::FromStr;

use async_trait::async_trait;
use domain::{aggregate::IssueBlockLinkAggregate, DomainEvent, IssueBlockLinkId, Version};

use sqlx::{any::AnyArguments, query::Query, Any, AnyPool, Transaction};
use use_case::{IssueBlockLinkRepository, IssueBlockLinkRepositoryError};

use crate::RdbConnectionPool;

use self::issue_block_link_id_row::IssueBlockLinkIdRow;

use super::event_store::{self, AggregateId, AggregateVersion, Event};

#[derive(Debug)]
pub struct SqliteIssueBlockLinkRepository {
    pool: AnyPool,
}

impl SqliteIssueBlockLinkRepository {
    pub async fn new(
        connection_pool: RdbConnectionPool,
    ) -> Result<Self, IssueBlockLinkRepositoryError> {
        Ok(Self {
            pool: AnyPool::from(connection_pool),
        })
    }

    async fn find_event_stream_id_by_issue_block_link_id(
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
        Ok(issue_block_link_id_row.map(|row| row.event_stream_id()))
    }

    async fn insert_issue_block_link_id(
        &self,
        transaction: &mut Transaction<'_, Any>,
        issue_block_link_id: &IssueBlockLinkId,
        event_stream_id: AggregateId,
    ) -> Result<(), IssueBlockLinkRepositoryError> {
        let query: Query<Any, AnyArguments> = sqlx::query(include_str!(
            "../../../sql/command/insert_issue_block_link_id.sql"
        ))
        .bind(issue_block_link_id.to_string())
        .bind(event_stream_id.to_string());
        let rows_affected = query
            .execute(transaction)
            .await
            .map_err(|e| IssueBlockLinkRepositoryError::Unknown(e.to_string()))?
            .rows_affected();
        if rows_affected != 1 {
            return Err(IssueBlockLinkRepositoryError::Unknown(
                "rows_affected != 1".to_string(),
            ));
        }

        Ok(())
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
            .find_event_stream_id_by_issue_block_link_id(&mut transaction, issue_block_link_id)
            .await?
        {
            Some(event_stream_id) => {
                let events =
                    event_store::find_events_by_event_stream_id(&mut transaction, event_stream_id)
                        .await
                        .map_err(|e| IssueBlockLinkRepositoryError::Unknown(e.to_string()))?;
                let mut issue_block_link_aggregate_events = vec![];
                for event in events {
                    let event = DomainEvent::from_str(event.data.as_str())
                        .map_err(|e| IssueBlockLinkRepositoryError::Unknown(e.to_string()))?;
                    // TODO: check dto.version and event_stream_id
                    issue_block_link_aggregate_events.push(
                        event.issue_block_link().ok_or_else(|| {
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
        issue_block_link: &IssueBlockLinkAggregate,
    ) -> Result<(), IssueBlockLinkRepositoryError> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|e| IssueBlockLinkRepositoryError::Unknown(e.to_string()))?;

        for event in issue_block_link.events() {
            let (issue_block_link_id, version) = event.key();
            if let Some(event_stream_id) = self
                .find_event_stream_id_by_issue_block_link_id(&mut transaction, issue_block_link_id)
                .await?
            {
                // update
                event_store::save(
                    &mut transaction,
                    version.prev().map(aggregate_version_from).transpose()?,
                    Event {
                        event_stream_id,
                        data: DomainEvent::from(event.clone()).to_string(),
                        version: aggregate_version_from(version)?,
                    },
                )
                .await
                .map_err(|e| IssueBlockLinkRepositoryError::Unknown(e.to_string()))?;
            } else {
                // create
                let event_stream_id = AggregateId::generate();
                event_store::save(
                    &mut transaction,
                    None,
                    Event {
                        event_stream_id,
                        data: DomainEvent::from(event.clone()).to_string(),
                        version: aggregate_version_from(version)?,
                    },
                )
                .await
                .map_err(|e| IssueBlockLinkRepositoryError::Unknown(e.to_string()))?;

                self.insert_issue_block_link_id(
                    &mut transaction,
                    issue_block_link_id,
                    event_stream_id,
                )
                .await?;
            }
        }
        transaction
            .commit()
            .await
            .map_err(|e| IssueBlockLinkRepositoryError::Unknown(e.to_string()))?;
        Ok(())
    }
}

fn aggregate_version_from(
    version: Version,
) -> Result<AggregateVersion, IssueBlockLinkRepositoryError> {
    Ok(AggregateVersion::from(
        u32::try_from(u64::from(version))
            .map_err(|e| IssueBlockLinkRepositoryError::Unknown(e.to_string()))?,
    ))
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
        let connection_pool = RdbConnectionPool::new(connection_uri.as_str()).await?;
        let repository = SqliteIssueBlockLinkRepository::new(connection_pool).await?;

        // save (create)
        let created = IssueBlockLinkAggregate::new(Instant::now(), "123".parse()?, "456".parse()?)?;
        repository.save(&created).await?;

        // find_by_id
        let found = repository.find_by_id(created.id()).await?;
        assert_eq!(Some(created.truncate_events()), found);
        let found = found.context("found is None")?;

        // save (update)
        let updated = found.unblock(Instant::now())?;
        repository.save(&updated).await?;
        let found = repository.find_by_id(updated.id()).await?;
        assert_eq!(Some(updated.truncate_events()), found);

        Ok(())
    }
}
