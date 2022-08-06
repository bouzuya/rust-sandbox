use domain::IssueCommentId;
use limited_date_time::Instant;

use crate::{
    issue_comment_repository::HasIssueCommentRepository, IssueCommentRepository,
    IssueManagementContextEvent,
};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[error("issue comment aggregate {0}")]
    IssueCommentAggregate(#[from] domain::aggregate::issue_comment::Error),
    #[error("issue comment not found {0}")]
    IssueCommentNotFound(IssueCommentId),
    #[error("issue comment repository {0}")]
    IssueCommentRepository(#[from] crate::use_case::issue_comment_repository::Error),
}

#[derive(Debug, Eq, PartialEq)]
pub struct DeleteIssueComment {
    pub issue_comment_id: IssueCommentId,
}

pub async fn delete_issue_comment<C: HasIssueCommentRepository + ?Sized>(
    context: &C,
    command: DeleteIssueComment,
) -> Result<IssueManagementContextEvent, Error> {
    // io
    let issue_comment = context
        .issue_comment_repository()
        .find_by_id(&command.issue_comment_id)
        .await?
        .ok_or(Error::IssueCommentNotFound(command.issue_comment_id))?;
    let at = Instant::now();

    // pure
    let deleted = issue_comment.delete(at)?;

    // io
    context.issue_comment_repository().save(&deleted).await?;

    let issue_comment_id = deleted
        .events()
        .iter()
        .next()
        .map(|event| event.issue_comment_id().to_owned())
        .expect("invalid event seq");
    Ok(IssueManagementContextEvent::IssueCommentDeleted { issue_comment_id })
}
