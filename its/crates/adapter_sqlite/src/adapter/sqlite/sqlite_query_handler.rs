use std::{fs, path::Path, str::FromStr};

use sqlx::{
    any::AnyConnectOptions,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    AnyPool,
};
use thiserror::Error;

// QueryIssue

#[derive(Clone, Debug)]
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

    pub async fn issue_list(&self) -> anyhow::Result<Vec<QueryIssue>> {
        Ok(vec![QueryIssue {
            id: "123".to_string(),
            status: "todo".to_string(),
            title: "title".to_string(),
            due: Some("2021-02-03T04:05:06Z".to_string()),
        }])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;

        // TODO: create command db
        // TODO: update query db
        let query_handler = SqliteQueryHandler::new(temp_dir.path()).await?;
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
