use domain::{
    aggregate::{
        issue::{attribute::IssueDue, IssueDescription, IssueTitle},
        IssueAggregate,
    },
    IssueNumber,
};
use limited_date_time::Instant;

use crate::{HasIssueRepository, IssueManagementContextEvent, IssueRepository};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("issue aggregate {0}")]
    IssueAggregate(#[from] domain::aggregate::issue::Error),
    #[error("issue repository {0}")]
    IssueRepository(#[from] crate::use_case::issue_repository::Error),
}

#[derive(Debug, Eq, PartialEq)]
pub struct CreateIssue {
    pub issue_title: IssueTitle,
    pub issue_due: Option<IssueDue>,
    pub issue_description: IssueDescription,
}

pub async fn create_issue<C: HasIssueRepository + ?Sized>(
    context: &C,
    command: CreateIssue,
) -> Result<IssueManagementContextEvent, Error> {
    // io
    let issue_number = context
        .issue_repository()
        .last_created()
        .await?
        .map(|issue| issue.id().issue_number().next_number())
        .unwrap_or_else(IssueNumber::start_number);
    let at = Instant::now();

    // pure
    let created = IssueAggregate::new(
        at,
        issue_number,
        command.issue_title,
        command.issue_due,
        command.issue_description,
    )?;

    // io
    context.issue_repository().save(&created).await?;

    let issue_id = created
        .events()
        .iter()
        .next()
        .map(|event| event.issue_id().to_owned())
        .expect("invalid event seq");
    Ok(IssueManagementContextEvent::IssueCreated { issue_id })
}
