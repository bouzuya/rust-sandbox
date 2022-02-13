// QueryIssue

#[derive(Clone, Debug)]
struct QueryIssue {
    id: String,
    status: String,
    title: String,
    due: Option<String>,
}

// SqliteQueryHandler

struct SqliteQueryHandler {}

impl SqliteQueryHandler {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {})
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
        // TODO: create command db
        // TODO: update query db
        let query_handler = SqliteQueryHandler::new().await?;
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
