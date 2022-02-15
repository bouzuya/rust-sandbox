use std::{fs, path::Path, str::FromStr};

use domain::aggregate::IssueAggregate;
use sqlx::{
    any::{AnyArguments, AnyConnectOptions},
    query::{Query, QueryAs},
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    Any, AnyPool, Executor, FromRow,
};
use thiserror::Error;

// QueryIssue

#[derive(Clone, Debug, FromRow)]
struct QueryIssue {
    id: String,
    status: String,
    title: String,
    due: Option<String>,
}

// QueryHandlerError

#[derive(Debug, Error)]
pub enum QueryHandlerError {
    #[error("Unknown {0}")]
    Unknown(String),
}

// SqliteQueryHandler

struct SqliteQueryHandler {
    pool: AnyPool,
}

impl SqliteQueryHandler {
    pub async fn new(data_dir: &Path) -> Result<Self, QueryHandlerError> {
        if !data_dir.exists() {
            fs::create_dir_all(data_dir).map_err(|e| QueryHandlerError::Unknown(e.to_string()))?;
        }
        let options = SqliteConnectOptions::from_str(&format!(
            "sqlite:{}?mode=rwc",
            data_dir
                .join("query.sqlite")
                .to_str()
                .ok_or_else(|| QueryHandlerError::Unknown("data_dir is not UTF-8".to_string()))?
        ))
        .map_err(|e| QueryHandlerError::Unknown(e.to_string()))?
        .journal_mode(SqliteJournalMode::Delete);
        let options = AnyConnectOptions::from(options);
        let pool = AnyPool::connect_with(options)
            .await
            .map_err(|e| QueryHandlerError::Unknown(e.to_string()))?;

        let mut transaction = pool
            .begin()
            .await
            .map_err(|e| QueryHandlerError::Unknown(e.to_string()))?;
        sqlx::query(include_str!("../../../sql/create_issues.sql"))
            .execute(&mut *transaction)
            .await
            .map_err(|e| QueryHandlerError::Unknown(e.to_string()))?;
        transaction
            .commit()
            .await
            .map_err(|e| QueryHandlerError::Unknown(e.to_string()))?;

        Ok(Self { pool })
    }

    pub async fn save_issue(&self, issue: IssueAggregate) -> anyhow::Result<()> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|e| QueryHandlerError::Unknown(e.to_string()))?;
        let query: Query<Any, AnyArguments> =
            sqlx::query(include_str!("../../../sql/query/delete_issue.sql"))
                .bind(issue.issue().id().to_string());
        query
            .execute(&mut transaction)
            .await
            .map_err(|e| QueryHandlerError::Unknown(e.to_string()))?;
        let query: Query<Any, AnyArguments> =
            sqlx::query(include_str!("../../../sql/query/insert_issue.sql"))
                .bind(issue.issue().id().to_string())
                .bind(issue.issue().status().to_string())
                .bind(issue.issue().title().to_string())
                .bind(issue.issue().due().map(|d| d.to_string()));
        query
            .execute(&mut transaction)
            .await
            .map_err(|e| QueryHandlerError::Unknown(e.to_string()))?;
        transaction
            .commit()
            .await
            .map_err(|e| QueryHandlerError::Unknown(e.to_string()))?;
        Ok(())
    }

    pub async fn issue_list(&self) -> anyhow::Result<Vec<QueryIssue>> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|e| QueryHandlerError::Unknown(e.to_string()))?;
        let issues: Vec<QueryIssue> =
            sqlx::query_as(include_str!("../../../sql/query/select_issues.sql"))
                .fetch_all(&mut transaction)
                .await?;
        Ok(issues)
    }
}

#[cfg(test)]
mod tests {
    use domain::aggregate::{IssueAggregateCommand, IssueAggregateCreateIssue};
    use limited_date_time::Instant;

    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;

        let (issue, _) = IssueAggregate::transaction(IssueAggregateCommand::Create(
            IssueAggregateCreateIssue {
                issue_number: "123".parse()?,
                issue_title: "title".parse()?,
                issue_due: Some("2021-02-03T04:05:06Z".parse()?),
                at: Instant::now(),
            },
        ))?;

        let query_handler = SqliteQueryHandler::new(temp_dir.path()).await?;

        query_handler.save_issue(issue.clone()).await?;
        query_handler.save_issue(issue).await?;

        let issues = query_handler.issue_list().await?;
        assert_eq!(1, issues.len());
        let issue = issues[0].clone();
        assert_eq!("123", issue.id);
        assert_eq!("todo", issue.status);
        assert_eq!("title", issue.title);
        assert_eq!(Some("2021-02-03T04:05:06Z".to_string()), issue.due);
        Ok(())
    }
}
