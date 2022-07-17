use domain::{aggregate::issue::attribute::IssueDue, IssueId};
use limited_date_time::Instant;

use crate::{HasIssueRepository, IssueManagementContextEvent, IssueRepository};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[error("issue aggregate {0}")]
    IssueAggregate(#[from] domain::aggregate::issue::Error),
    #[error("issue not found {0}")]
    IssueNotFound(IssueId),
    #[error("issue repository {0}")]
    IssueRepository(#[from] crate::use_case::issue_repository::Error),
}

#[derive(Debug, Eq, PartialEq)]
pub struct UpdateIssue {
    pub issue_id: IssueId,
    pub issue_due: Option<IssueDue>,
}

pub async fn update_issue<C: HasIssueRepository + ?Sized>(
    context: &C,
    command: UpdateIssue,
) -> Result<IssueManagementContextEvent, Error> {
    // io
    let issue = context
        .issue_repository()
        .find_by_id(&command.issue_id)
        .await?
        .ok_or(Error::IssueNotFound(command.issue_id))?;
    let issue_due = command.issue_due;
    let at = Instant::now();

    // pure
    let updated = issue.update(issue_due, at)?;

    // io
    context.issue_repository().save(&updated).await?;

    let issue_id = updated
        .events()
        .iter()
        .next()
        .map(|event| event.issue_id().to_owned())
        .expect("invalid event seq");
    Ok(IssueManagementContextEvent::IssueUpdated { issue_id })
}
