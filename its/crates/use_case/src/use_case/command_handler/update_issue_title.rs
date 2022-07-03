use domain::{DomainEvent, IssueId, IssueTitle};
use limited_date_time::Instant;

use crate::{
    HasIssueRepository, IssueManagementContextEvent, IssueRepository, IssueRepositoryError,
};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[error("issue aggregate {0}")]
    IssueAggregate(#[from] domain::aggregate::IssueAggregateError),
    #[error("issue not found {0}")]
    IssueNotFound(IssueId),
    #[error("issue repository {0}")]
    IssueRepository(#[from] IssueRepositoryError),
}

#[derive(Debug, Eq, PartialEq)]
pub struct UpdateIssueTitle {
    pub issue_id: IssueId,
    pub issue_title: IssueTitle,
}

pub async fn update_issue_title<C: HasIssueRepository + ?Sized>(
    context: &C,
    command: UpdateIssueTitle,
) -> Result<Vec<IssueManagementContextEvent>, Error> {
    // io
    let issue = context
        .issue_repository()
        .find_by_id(&command.issue_id)
        .await?
        .ok_or(Error::IssueNotFound(command.issue_id))?;
    let issue_title = command.issue_title;
    let at = Instant::now();

    // pure
    let updated = issue.update_title(issue_title, at)?;

    // io
    context.issue_repository().save(&updated).await?;

    Ok(updated
        .events()
        .iter()
        .cloned()
        .map(DomainEvent::from)
        .map(IssueManagementContextEvent::from)
        .collect::<Vec<IssueManagementContextEvent>>())
}