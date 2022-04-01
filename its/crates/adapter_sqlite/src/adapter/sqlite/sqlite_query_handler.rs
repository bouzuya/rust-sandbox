use std::{
    fmt::Debug,
    str::FromStr,
    sync::{Arc, Mutex},
};

use domain::{
    aggregate::{IssueAggregate, IssueBlockLinkAggregate},
    IssueId,
};
use serde::Serialize;
use sqlx::{
    any::{AnyArguments, AnyConnectOptions},
    query::Query,
    Any, AnyPool, FromRow,
};
use thiserror::Error;
use use_case::IssueRepository;

// QueryIssue

#[derive(Clone, Debug, Eq, FromRow, PartialEq, Serialize)]
pub struct QueryIssue {
    pub id: String,
    pub status: String,
    pub title: String,
    pub due: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct QueryIssueIdWithTitle {
    pub id: String,
    pub title: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct QueryIssueWithLinks {
    pub id: String,
    pub status: String,
    pub title: String,
    pub due: Option<String>,
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

#[derive(Debug, Error)]
pub enum QueryHandlerError {
    #[error("Unknown {0}")]
    Unknown(String),
}

impl From<sqlx::Error> for QueryHandlerError {
    fn from(e: sqlx::Error) -> Self {
        Self::Unknown(e.to_string())
    }
}

// SqliteQueryHandler

pub struct SqliteQueryHandler {
    pool: AnyPool,
    issue_repository: Arc<Mutex<dyn IssueRepository + Send + Sync>>,
}

impl SqliteQueryHandler {
    pub async fn new(
        connection_uri: &str,
        issue_repository: Arc<Mutex<dyn IssueRepository + Send + Sync>>,
    ) -> Result<Self, QueryHandlerError> {
        let options = AnyConnectOptions::from_str(connection_uri)?;
        let pool = AnyPool::connect_with(options).await?;

        let mut transaction = pool.begin().await?;
        sqlx::query(include_str!("../../../sql/query/create_issues.sql"))
            .execute(&mut *transaction)
            .await?;
        sqlx::query(include_str!(
            "../../../sql/query/create_issue_block_links.sql"
        ))
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;

        Ok(Self {
            pool,
            issue_repository,
        })
    }

    pub async fn save_issue(&self, issue: IssueAggregate) -> Result<(), QueryHandlerError> {
        let mut transaction = self.pool.begin().await?;
        let query: Query<Any, AnyArguments> =
            sqlx::query(include_str!("../../../sql/query/delete_issue.sql"))
                .bind(issue.id().to_string());
        query.execute(&mut transaction).await?;
        let query: Query<Any, AnyArguments> =
            sqlx::query(include_str!("../../../sql/query/insert_issue.sql"))
                .bind(issue.id().to_string())
                .bind(issue.status().to_string())
                .bind(issue.title().to_string())
                .bind(issue.due().map(|d| d.to_string()));
        query.execute(&mut transaction).await?;
        transaction.commit().await?;
        Ok(())
    }

    pub async fn save_issue_block_link(
        &self,
        issue_block_link: IssueBlockLinkAggregate,
    ) -> Result<(), QueryHandlerError> {
        let mut transaction = self.pool.begin().await?;
        let query: Query<Any, AnyArguments> = sqlx::query(include_str!(
            "../../../sql/query/delete_issue_block_link.sql"
        ))
        .bind(issue_block_link.id().issue_id().to_string())
        .bind(issue_block_link.id().blocked_issue_id().to_string());
        query.execute(&mut transaction).await?;

        let issue_repository = self.issue_repository.lock().map_err(|e| {
            QueryHandlerError::Unknown(format!("IssueRepository can't lock: {}", e))
        })?;
        let issue_title = issue_repository
            .find_by_id(issue_block_link.id().issue_id())
            .await
            .map_err(|e| QueryHandlerError::Unknown(e.to_string()))?
            .ok_or_else(|| QueryHandlerError::Unknown("no issue".to_string()))?
            .title()
            .to_string();
        let blocked_issue_title = issue_repository
            .find_by_id(issue_block_link.id().blocked_issue_id())
            .await
            .map_err(|e| QueryHandlerError::Unknown(e.to_string()))?
            .ok_or_else(|| QueryHandlerError::Unknown("no issue".to_string()))?
            .title()
            .to_string();
        let query: Query<Any, AnyArguments> = sqlx::query(include_str!(
            "../../../sql/query/insert_issue_block_link.sql"
        ))
        .bind(issue_block_link.id().issue_id().to_string())
        .bind(issue_title.to_string())
        .bind(issue_block_link.id().blocked_issue_id().to_string())
        .bind(blocked_issue_title.to_string());
        let rows_affected = query.execute(&mut transaction).await?.rows_affected();
        if rows_affected != 1 {
            return Err(QueryHandlerError::Unknown("rows_affected != 1".to_string()));
        }

        transaction.commit().await?;
        Ok(())
    }

    pub async fn issue_list(&self) -> Result<Vec<QueryIssue>, QueryHandlerError> {
        let mut transaction = self.pool.begin().await?;
        let issues: Vec<QueryIssue> =
            sqlx::query_as(include_str!("../../../sql/query/select_issues.sql"))
                .fetch_all(&mut transaction)
                .await?;
        Ok(issues)
    }

    pub async fn issue_view(
        &self,
        issue_id: &IssueId,
    ) -> Result<Option<QueryIssueWithLinks>, QueryHandlerError> {
        let mut transaction = self.pool.begin().await?;
        let issue: Option<QueryIssue> =
            sqlx::query_as(include_str!("../../../sql/query/select_issue.sql"))
                .bind(issue_id.to_string())
                .fetch_optional(&mut transaction)
                .await?;
        match issue {
            Some(issue) => {
                let blocks: Vec<QueryIssueBlockLink> = sqlx::query_as(include_str!(
                    "../../../sql/query/select_issue_block_links_by_issue_id.sql"
                ))
                .bind(issue_id.to_string())
                .fetch_all(&mut transaction)
                .await?;
                let is_blocked_by: Vec<QueryIssueBlockLink> = sqlx::query_as(include_str!(
                    "../../../sql/query/select_issue_block_links_by_blocked_issue_id.sql"
                ))
                .bind(issue_id.to_string())
                .fetch_all(&mut transaction)
                .await?;
                Ok(Some(QueryIssueWithLinks {
                    id: issue.id,
                    status: issue.status,
                    title: issue.title,
                    due: issue.due,
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
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use anyhow::Context;
    use limited_date_time::Instant;

    use crate::{RdbConnectionPool, SqliteIssueRepository};

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
        )?;

        let issue_repository = SqliteIssueRepository::new(connection_pool).await?;
        issue_repository.save(&issue).await?;

        let query_handler = SqliteQueryHandler::new(
            &query_connection_uri,
            Arc::new(Mutex::new(issue_repository)),
        )
        .await?;

        query_handler.save_issue(issue).await?;

        let issues = query_handler.issue_list().await?;
        assert_eq!(1, issues.len());
        let issue = issues[0].clone();
        assert_eq!("123", issue.id);
        assert_eq!("todo", issue.status);
        assert_eq!("title", issue.title);
        assert_eq!(Some("2021-02-03T04:05:06Z".to_string()), issue.due);

        let found = query_handler.issue_view(&"123".parse()?).await?;
        assert_eq!(
            Some(QueryIssueWithLinks {
                id: "123".to_string(),
                status: "todo".to_string(),
                title: "title".to_string(),
                due: Some("2021-02-03T04:05:06Z".to_string()),
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

        let issue1 = IssueAggregate::new(Instant::now(), "1".parse()?, "title1".parse()?, None)?;
        let issue2 = IssueAggregate::new(Instant::now(), "2".parse()?, "title2".parse()?, None)?;
        let issue3 = IssueAggregate::new(Instant::now(), "3".parse()?, "title3".parse()?, None)?;
        let issue_block_link1 = issue1.block(issue2.clone(), Instant::now())?;
        let issue_block_link2 = issue2.block(issue3.clone(), Instant::now())?;

        let issue_repository = SqliteIssueRepository::new(connection_pool).await?;
        issue_repository.save(&issue1).await?;
        issue_repository.save(&issue2).await?;
        issue_repository.save(&issue3).await?;

        let query_handler = SqliteQueryHandler::new(
            &query_connection_uri,
            Arc::new(Mutex::new(issue_repository)),
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
                status: "todo".to_string(),
                title: "title2".to_string(),
                due: None,
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
