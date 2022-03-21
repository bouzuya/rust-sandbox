use std::{
    fmt::Debug,
    fs,
    path::Path,
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
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
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

#[derive(Debug)]
pub struct SqliteQueryHandler {
    pool: AnyPool,
    issue_repository: Arc<Mutex<dyn IssueRepository>>,
}

impl SqliteQueryHandler {
    pub async fn new(
        data_dir: &Path,
        issue_repository: Arc<Mutex<dyn IssueRepository>>,
    ) -> Result<Self, QueryHandlerError> {
        if !data_dir.exists() {
            fs::create_dir_all(data_dir).map_err(|e| QueryHandlerError::Unknown(e.to_string()))?;
        }
        let options = SqliteConnectOptions::from_str(&format!(
            "sqlite:{}?mode=rwc",
            data_dir
                .join("query.sqlite")
                .to_str()
                .ok_or_else(|| QueryHandlerError::Unknown("data_dir is not UTF-8".to_string()))?
        ))?
        .journal_mode(SqliteJournalMode::Delete);
        let options = AnyConnectOptions::from(options);
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
        // TODO: insert
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
    ) -> Result<Option<QueryIssue>, QueryHandlerError> {
        let mut transaction = self.pool.begin().await?;
        let issue: Option<QueryIssue> =
            sqlx::query_as(include_str!("../../../sql/query/select_issue.sql"))
                .bind(issue_id.to_string())
                .fetch_optional(&mut transaction)
                .await?;
        Ok(issue)
    }
}

#[cfg(test)]
mod tests {
    use limited_date_time::Instant;

    use crate::{SqliteConnectionPool, SqliteIssueRepository};

    use super::*;

    #[tokio::test]
    async fn issue_test() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let sqlite_dir = temp_dir.path().join("its");
        let connection_pool = SqliteConnectionPool::new(sqlite_dir.clone()).await?;

        let issue = IssueAggregate::new(
            Instant::now(),
            "123".parse()?,
            "title".parse()?,
            Some("2021-02-03T04:05:06Z".parse()?),
        )?;

        let issue_repository = SqliteIssueRepository::new(connection_pool).await?;
        let query_handler =
            SqliteQueryHandler::new(temp_dir.path(), Arc::new(Mutex::new(issue_repository)))
                .await?;

        query_handler.save_issue(issue.clone()).await?;
        query_handler.save_issue(issue).await?;

        let issues = query_handler.issue_list().await?;
        assert_eq!(1, issues.len());
        let issue = issues[0].clone();
        assert_eq!("123", issue.id);
        assert_eq!("todo", issue.status);
        assert_eq!("title", issue.title);
        assert_eq!(Some("2021-02-03T04:05:06Z".to_string()), issue.due);

        let found = query_handler.issue_view(&"123".parse()?).await?;
        assert_eq!(Some(issue), found);
        Ok(())
    }

    #[tokio::test]
    async fn issue_block_link_test() {
        // TODO:
    }
}
