use domain::{aggregate::IssueAggregate, DomainEvent, IssueDue, IssueNumber, IssueTitle};
use limited_date_time::Instant;

use crate::{
    HasIssueRepository, IssueManagementContextEvent, IssueRepository, IssueRepositoryError,
};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("issue aggregate {0}")]
    IssueAggregate(#[from] domain::aggregate::IssueAggregateError),
    #[error("issue repository {0}")]
    IssueRepository(#[from] IssueRepositoryError),
}

#[derive(Debug)]
pub struct CreateIssue {
    pub issue_title: IssueTitle,
    pub issue_due: Option<IssueDue>,
}

pub async fn create_issue<C: HasIssueRepository + ?Sized>(
    context: &C,
    command: CreateIssue,
) -> Result<Vec<IssueManagementContextEvent>, Error> {
    // io
    let issue_number = context
        .issue_repository()
        .last_created()
        .await?
        .map(|issue| issue.id().issue_number().next_number())
        .unwrap_or_else(IssueNumber::start_number);
    let at = Instant::now();

    // pure
    let created = IssueAggregate::new(at, issue_number, command.issue_title, command.issue_due)?;

    // io
    context.issue_repository().save(&created).await?;

    Ok(created
        .events()
        .iter()
        .cloned()
        .map(DomainEvent::from)
        .map(IssueManagementContextEvent::from)
        .collect::<Vec<IssueManagementContextEvent>>())
}
