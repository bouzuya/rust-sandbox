use std::{
    fmt::Debug,
    str::FromStr,
    sync::{Arc, Mutex},
};

use adapter_sqlite::RdbConnectionPool;
use domain::{
    aggregate::{
        IssueAggregate, IssueAggregateEvent, IssueBlockLinkAggregate, IssueBlockLinkAggregateEvent,
    },
    DomainEvent, IssueId, ParseDomainEventError,
};
use event_store::{Event, EventId};
use serde::Serialize;
use sqlx::{
    any::AnyArguments, migrate::Migrator, query::Query, Any, AnyPool, FromRow, Transaction,
};
use use_case::{IssueBlockLinkRepository, IssueRepository};

use super::query_migration_source::QueryMigrationSource;

pub type Result<T, E = Error> = std::result::Result<T, E>;

// QueryIssue

#[derive(Clone, Debug, Eq, FromRow, PartialEq, Serialize)]
pub struct QueryIssue {
    pub id: String,
    pub resolution: Option<String>,
    pub status: String,
    pub title: String,
    pub due: Option<String>,
    pub description: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct QueryIssueIdWithTitle {
    pub id: String,
    pub title: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct QueryIssueWithLinks {
    pub id: String,
    pub resolution: Option<String>,
    pub status: String,
    pub title: String,
    pub due: Option<String>,
    pub description: String,
    pub blocks: Vec<QueryIssueIdWithTitle>,
    pub is_blocked_by: Vec<QueryIssueIdWithTitle>,
}

#[derive(Clone, Debug, Eq, FromRow, PartialEq, Serialize)]
pub struct QueryIssueBlockLink {
    pub issue_id: String,
    pub issue_title: String,
    pub blocked_issue_id: String,
    pub blocked_issue_title: String,
}

// QueryHandlerError

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown {0}")]
    Unknown(String),
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::Unknown(e.to_string())
    }
}

impl From<sqlx::migrate::MigrateError> for Error {
    fn from(e: sqlx::migrate::MigrateError) -> Self {
        Self::Unknown(e.to_string())
    }
}

impl From<event_store::Error> for Error {
    fn from(e: event_store::Error) -> Self {
        Self::Unknown(e.to_string())
    }
}

// SqliteQueryHandler

pub struct SqliteQueryHandler {
    event_store_pool: AnyPool,
    query_pool: AnyPool,
    issue_repository: Arc<Mutex<dyn IssueRepository + Send + Sync>>,
}

impl SqliteQueryHandler {
    pub async fn new(
        connection_uri: &str,
        event_store_pool: RdbConnectionPool,
        issue_repository: Arc<Mutex<dyn IssueRepository + Send + Sync>>,
        _issue_block_link_repository: Arc<Mutex<dyn IssueBlockLinkRepository + Send + Sync>>,
    ) -> Result<Self> {
        let query_pool = AnyPool::connect(connection_uri).await?;
        let created = Self {
            event_store_pool: AnyPool::from(event_store_pool),
            query_pool,
            issue_repository,
        };

        created.create_database().await?;

        Ok(created)
    }

    pub async fn update_database(&self) -> Result<()> {
        let mut query_transaction = self.query_pool.begin().await?;

        #[derive(FromRow)]
        struct LastEventIdRow {
            event_id: String,
        }
        let row: Option<LastEventIdRow> =
            sqlx::query_as(include_str!("../../../sql/select_last_event_id.sql"))
                .fetch_optional(&mut query_transaction)
                .await?;
        let mut event_id = row
            .map(|r| r.event_id)
            .and_then(|s| EventId::from_str(s.as_str()).ok());

        let mut event_store_transaction = self.event_store_pool.begin().await?;
        let events = match event_id {
            Some(event_id) => {
                event_store::find_events_by_event_id_after(&mut event_store_transaction, event_id)
                    .await?
            }
            None => event_store::find_events(&mut event_store_transaction).await?,
        };
        for event in events {
            let id = event.id;
            let mut event_store_transaction = self.event_store_pool.begin().await?;
            self.handle_event(&mut event_store_transaction, event)
                .await?;
            event_store_transaction.commit().await?;
            self.save_last_event_id(id, event_id).await?;
            event_id = Some(id);
        }

        Ok(())
    }

    async fn save_last_event_id(
        &self,
        new_event_id: EventId,
        old_event_id: Option<EventId>,
    ) -> Result<()> {
        let mut query_transaction = self.query_pool.begin().await?;
        match old_event_id {
            Some(event_id) => {
                let query: Query<Any, AnyArguments> =
                    sqlx::query(include_str!("../../../sql/update_last_event_id.sql"))
                        .bind(new_event_id.to_string())
                        .bind(event_id.to_string());
                let rows_affected = query.execute(&mut query_transaction).await?.rows_affected();
                if rows_affected != 1 {
                    return Err(Error::Unknown(
                        "update_last_event_id rows_affected != 1".to_string(),
                    ));
                }
            }
            None => {
                let query: Query<Any, AnyArguments> =
                    sqlx::query(include_str!("../../../sql/insert_last_event_id.sql"))
                        .bind(new_event_id.to_string());
                query.execute(&mut query_transaction).await?;
            }
        }
        query_transaction.commit().await?;
        Ok(())
    }

    pub async fn create_database(&self) -> Result<()> {
        let mut query_transaction = self.query_pool.begin().await?;
        let migrator = Migrator::new(QueryMigrationSource::default()).await?;
        migrator.run(&mut *query_transaction).await?;
        query_transaction.commit().await?;
        Ok(())
    }

    pub async fn drop_database(&self) -> Result<()> {
        let mut query_transaction = self.query_pool.begin().await?;
        let sqls = vec![
            include_str!("../../../sql/drop_issue_block_links.sql"),
            include_str!("../../../sql/drop_issues.sql"),
            include_str!("../../../sql/drop_last_event_id.sql"),
        ];
        for sql in sqls {
            sqlx::query(sql).execute(&mut *query_transaction).await?;
        }
        query_transaction.commit().await?;
        Ok(())
    }

    pub async fn reset_database(&self) -> Result<()> {
        self.drop_database().await?;
        self.create_database().await?;
        self.update_database().await?;
        Ok(())
    }

    pub async fn save_issue(&self, issue: IssueAggregate) -> Result<()> {
        let mut query_transaction = self.query_pool.begin().await?;
        let query: Query<Any, AnyArguments> =
            sqlx::query(include_str!("../../../sql/delete_issue.sql")).bind(issue.id().to_string());
        query.execute(&mut query_transaction).await?;
        let query: Query<Any, AnyArguments> =
            sqlx::query(include_str!("../../../sql/insert_issue.sql"))
                .bind(issue.id().to_string())
                .bind(issue.resolution().map(|s| s.to_string()))
                .bind(issue.status().to_string())
                .bind(issue.title().to_string())
                .bind(issue.due().map(|d| d.to_string()))
                .bind(issue.description().to_string());
        query.execute(&mut query_transaction).await?;
        query_transaction.commit().await?;
        Ok(())
    }

    pub async fn save_issue_block_link(
        &self,
        issue_block_link: IssueBlockLinkAggregate,
    ) -> Result<()> {
        let mut query_transaction = self.query_pool.begin().await?;
        let query: Query<Any, AnyArguments> =
            sqlx::query(include_str!("../../../sql/delete_issue_block_link.sql"))
                .bind(issue_block_link.id().issue_id().to_string())
                .bind(issue_block_link.id().blocked_issue_id().to_string());
        query.execute(&mut query_transaction).await?;

        // FIXME
        let issue_repository = self
            .issue_repository
            .lock()
            .map_err(|e| Error::Unknown(format!("IssueRepository can't lock: {}", e)))?;
        let issue_title = issue_repository
            .find_by_id(issue_block_link.id().issue_id())
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?
            .ok_or_else(|| Error::Unknown("no issue".to_string()))?
            .title()
            .to_string();
        let blocked_issue_title = issue_repository
            .find_by_id(issue_block_link.id().blocked_issue_id())
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?
            .ok_or_else(|| Error::Unknown("no issue".to_string()))?
            .title()
            .to_string();
        let query: Query<Any, AnyArguments> =
            sqlx::query(include_str!("../../../sql/insert_issue_block_link.sql"))
                .bind(issue_block_link.id().issue_id().to_string())
                .bind(issue_title.to_string())
                .bind(issue_block_link.id().blocked_issue_id().to_string())
                .bind(blocked_issue_title.to_string());
        let rows_affected = query.execute(&mut query_transaction).await?.rows_affected();
        if rows_affected != 1 {
            return Err(Error::Unknown("rows_affected != 1".to_string()));
        }

        query_transaction.commit().await?;
        Ok(())
    }

    pub async fn issue_list(&self) -> Result<Vec<QueryIssue>> {
        let mut query_transaction = self.query_pool.begin().await?;
        let issues: Vec<QueryIssue> =
            sqlx::query_as(include_str!("../../../sql/select_issues.sql"))
                .fetch_all(&mut query_transaction)
                .await?;
        Ok(issues)
    }

    pub async fn issue_view(&self, issue_id: &IssueId) -> Result<Option<QueryIssueWithLinks>> {
        let mut query_transaction = self.query_pool.begin().await?;
        let issue: Option<QueryIssue> =
            sqlx::query_as(include_str!("../../../sql/select_issue.sql"))
                .bind(issue_id.to_string())
                .fetch_optional(&mut query_transaction)
                .await?;
        match issue {
            Some(issue) => {
                let blocks: Vec<QueryIssueBlockLink> = sqlx::query_as(include_str!(
                    "../../../sql/select_issue_block_links_by_issue_id.sql"
                ))
                .bind(issue_id.to_string())
                .fetch_all(&mut query_transaction)
                .await?;
                let is_blocked_by: Vec<QueryIssueBlockLink> = sqlx::query_as(include_str!(
                    "../../../sql/select_issue_block_links_by_blocked_issue_id.sql"
                ))
                .bind(issue_id.to_string())
                .fetch_all(&mut query_transaction)
                .await?;
                Ok(Some(QueryIssueWithLinks {
                    id: issue.id,
                    resolution: issue.resolution,
                    status: issue.status,
                    title: issue.title,
                    due: issue.due,
                    description: issue.description,
                    blocks: blocks
                        .into_iter()
                        .map(|issue_block_link| QueryIssueIdWithTitle {
                            id: issue_block_link.blocked_issue_id,
                            title: issue_block_link.blocked_issue_title,
                        })
                        .collect::<Vec<QueryIssueIdWithTitle>>(),
                    is_blocked_by: is_blocked_by
                        .into_iter()
                        .map(|issue_block_link| QueryIssueIdWithTitle {
                            id: issue_block_link.issue_id,
                            title: issue_block_link.issue_title,
                        })
                        .collect::<Vec<QueryIssueIdWithTitle>>(),
                }))
            }
            None => Ok(None),
        }
    }

    async fn handle_event(
        &self,
        event_store_transaction: &mut Transaction<'_, Any>,
        event: Event,
    ) -> Result<()> {
        let domain_event = DomainEvent::from_str(event.data.as_str())
            .map_err(|e| Error::Unknown(e.to_string()))?;
        match domain_event {
            DomainEvent::Issue(_) => {
                // TODO: improve
                let events =
                    event_store::find_events_by_event_stream_id_and_version_less_than_equal(
                        event_store_transaction,
                        event.stream_id,
                        event.stream_seq,
                    )
                    .await?
                    .into_iter()
                    .map(|e| DomainEvent::from_str(e.data.as_str()))
                    .collect::<Result<Vec<DomainEvent>, ParseDomainEventError>>()
                    .map_err(|e| Error::Unknown(e.to_string()))?
                    .into_iter()
                    .filter_map(|e| e.issue())
                    .collect::<Vec<IssueAggregateEvent>>();
                let issue = IssueAggregate::from_events(&events)
                    .map_err(|e| Error::Unknown(e.to_string()))?;
                self.save_issue(issue).await?;
            }
            DomainEvent::IssueBlockLink(_) => {
                // TODO: improve
                let events =
                    event_store::find_events_by_event_stream_id_and_version_less_than_equal(
                        event_store_transaction,
                        event.stream_id,
                        event.stream_seq,
                    )
                    .await?
                    .into_iter()
                    .map(|e| DomainEvent::from_str(e.data.as_str()))
                    .collect::<Result<Vec<DomainEvent>, ParseDomainEventError>>()
                    .map_err(|e| Error::Unknown(e.to_string()))?
                    .into_iter()
                    .filter_map(|e| e.issue_block_link())
                    .collect::<Vec<IssueBlockLinkAggregateEvent>>();
                let issue_block_link = IssueBlockLinkAggregate::from_events(&events)
                    .map_err(|e| Error::Unknown(e.to_string()))?;
                self.save_issue_block_link(issue_block_link).await?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use anyhow::Context;
    use limited_date_time::Instant;

    use super::*;

    #[tokio::test]
    async fn issue_test() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let sqlite_dir = temp_dir.path().join("its");
        let data_dir = sqlite_dir;
        if !data_dir.exists() {
            fs::create_dir_all(data_dir.as_path())?;
        }
        let new_connection_uri = |path: PathBuf| -> anyhow::Result<String> {
            Ok(format!(
                "sqlite:{}?mode=rwc",
                path.to_str().context("path is not utf-8")?
            ))
        };
        let command_connection_uri = new_connection_uri(data_dir.join("command.sqlite"))?;
        let query_connection_uri = new_connection_uri(data_dir.join("query.sqlite"))?;

        let connection_pool = RdbConnectionPool::new(&command_connection_uri).await?;

        let issue = IssueAggregate::new(
            Instant::now(),
            "123".parse()?,
            "title".parse()?,
            Some("2021-02-03T04:05:06Z".parse()?),
            "desc1".parse()?,
        )?;

        let issue_repository = connection_pool.issue_repository()?;
        issue_repository.save(&issue).await?;
        let issue_block_link_repository = connection_pool.issue_block_link_repository()?;

        let query_handler = SqliteQueryHandler::new(
            &query_connection_uri,
            connection_pool,
            Arc::new(Mutex::new(issue_repository)),
            Arc::new(Mutex::new(issue_block_link_repository)),
        )
        .await?;

        query_handler.save_issue(issue).await?;

        let issues = query_handler.issue_list().await?;
        assert_eq!(1, issues.len());
        let issue = issues[0].clone();
        assert_eq!("123", issue.id);
        assert_eq!(None, issue.resolution);
        assert_eq!("todo", issue.status);
        assert_eq!("title", issue.title);
        assert_eq!(Some("2021-02-03T04:05:06Z".to_string()), issue.due);

        let found = query_handler.issue_view(&"123".parse()?).await?;
        assert_eq!(
            Some(QueryIssueWithLinks {
                id: "123".to_string(),
                resolution: None,
                status: "todo".to_string(),
                title: "title".to_string(),
                due: Some("2021-02-03T04:05:06Z".to_string()),
                description: "desc1".to_string(),
                blocks: vec![],
                is_blocked_by: vec![]
            }),
            found
        );
        Ok(())
    }

    #[tokio::test]
    async fn issue_block_link_test() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let sqlite_dir = temp_dir.path().join("its");
        let data_dir = sqlite_dir;
        if !data_dir.exists() {
            fs::create_dir_all(data_dir.as_path())?;
        }
        let new_connection_uri = |path: PathBuf| -> anyhow::Result<String> {
            Ok(format!(
                "sqlite:{}?mode=rwc",
                path.to_str().context("path is not utf-8")?
            ))
        };
        let command_connection_uri = new_connection_uri(data_dir.join("command.sqlite"))?;
        let query_connection_uri = new_connection_uri(data_dir.join("query.sqlite"))?;
        let connection_pool = RdbConnectionPool::new(&command_connection_uri).await?;

        let issue1 = IssueAggregate::new(
            Instant::now(),
            "1".parse()?,
            "title1".parse()?,
            None,
            "desc1".parse()?,
        )?;
        let issue2 = IssueAggregate::new(
            Instant::now(),
            "2".parse()?,
            "title2".parse()?,
            None,
            "desc2".parse()?,
        )?;
        let issue3 = IssueAggregate::new(
            Instant::now(),
            "3".parse()?,
            "title3".parse()?,
            None,
            "desc3".parse()?,
        )?;
        let issue_block_link1 = issue1.block(issue2.clone(), Instant::now())?;
        let issue_block_link2 = issue2.block(issue3.clone(), Instant::now())?;

        let issue_repository = connection_pool.issue_repository()?;
        issue_repository.save(&issue1).await?;
        issue_repository.save(&issue2).await?;
        issue_repository.save(&issue3).await?;
        let issue_block_link_repository = connection_pool.issue_block_link_repository()?;

        let query_handler = SqliteQueryHandler::new(
            &query_connection_uri,
            connection_pool,
            Arc::new(Mutex::new(issue_repository)),
            Arc::new(Mutex::new(issue_block_link_repository)),
        )
        .await?;

        query_handler.save_issue(issue1).await?;
        query_handler.save_issue(issue2).await?;
        query_handler.save_issue(issue3).await?;
        query_handler
            .save_issue_block_link(issue_block_link1)
            .await?;
        query_handler
            .save_issue_block_link(issue_block_link2)
            .await?;

        let found = query_handler.issue_view(&"2".parse()?).await?;
        assert_eq!(
            Some(QueryIssueWithLinks {
                id: "2".to_string(),
                resolution: None,
                status: "todo".to_string(),
                title: "title2".to_string(),
                due: None,
                description: "desc2".to_string(),
                blocks: vec![QueryIssueIdWithTitle {
                    id: "3".to_string(),
                    title: "title3".to_string(),
                }],
                is_blocked_by: vec![QueryIssueIdWithTitle {
                    id: "1".to_string(),
                    title: "title1".to_string(),
                }]
            }),
            found
        );
        Ok(())
    }
}
