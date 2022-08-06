use domain::{
    aggregate::issue_comment::{attribute::IssueCommentText, IssueCommentAggregate},
    IssueCommentId, IssueId,
};
use limited_date_time::Instant;

use crate::{
    issue_comment_repository::HasIssueCommentRepository, HasIssueRepository,
    IssueCommentRepository, IssueManagementContextEvent, IssueRepository,
};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[error("issue comment aggregate {0}")]
    IssueCommentAggregate(#[from] domain::aggregate::issue_comment::Error),
    #[error("issue comment repository {0}")]
    IssueCommentRepository(#[from] crate::use_case::issue_comment_repository::Error),
    #[error("issue not found {0}")]
    IssueNotFound(IssueId),
    #[error("issue repository {0}")]
    IssueRepository(#[from] crate::use_case::issue_repository::Error),
}

#[derive(Debug, Eq, PartialEq)]
pub struct CreateIssueComment {
    pub issue_id: IssueId,
    pub text: IssueCommentText,
}

pub async fn create_issue_comment<C: HasIssueRepository + HasIssueCommentRepository + ?Sized>(
    context: &C,
    command: CreateIssueComment,
) -> Result<IssueManagementContextEvent, Error> {
    // io
    let issue = context
        .issue_repository()
        .find_by_id(&command.issue_id)
        .await?
        .ok_or(Error::IssueNotFound(command.issue_id))?;
    let text = command.text;
    let issue_comment_id = IssueCommentId::generate();
    let issue_id = issue.id().clone();
    let at = Instant::now();

    // pure
    let created = IssueCommentAggregate::new(at, issue_comment_id, issue_id, text)?;

    // io
    context.issue_comment_repository().save(&created).await?;

    let issue_comment_id = created
        .events()
        .iter()
        .next()
        .map(|event| event.issue_comment_id().to_owned())
        .expect("invalid event seq");
    Ok(IssueManagementContextEvent::IssueCommentCreated { issue_comment_id })
}
