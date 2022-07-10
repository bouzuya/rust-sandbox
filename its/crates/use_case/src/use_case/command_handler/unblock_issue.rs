use domain::IssueBlockLinkId;
use limited_date_time::Instant;

use crate::{
    HasIssueBlockLinkRepository, IssueBlockLinkRepository, IssueBlockLinkRepositoryError,
    IssueManagementContextEvent,
};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[error("issue block link aggregate {0}")]
    IssueBlockLinkAggregate(#[from] domain::aggregate::issue_block_link::Error),
    #[error("issue block link repository {0}")]
    IssueBlockLinkRepository(#[from] IssueBlockLinkRepositoryError),
    #[error("issue block link not found {0}")]
    IssueBlockLinkNotFound(IssueBlockLinkId),
}

#[derive(Debug, Eq, PartialEq)]
pub struct UnblockIssue {
    pub issue_block_link_id: IssueBlockLinkId,
}

pub async fn unblock_issue<C: HasIssueBlockLinkRepository + ?Sized>(
    context: &C,
    UnblockIssue {
        issue_block_link_id,
    }: UnblockIssue,
) -> Result<IssueManagementContextEvent, Error> {
    // io
    let at = Instant::now();
    let issue_block_link = context
        .issue_block_link_repository()
        .find_by_id(&issue_block_link_id)
        .await?
        .ok_or(Error::IssueBlockLinkNotFound(issue_block_link_id))?;

    // pure
    let updated = issue_block_link.unblock(at)?;

    // io
    context.issue_block_link_repository().save(&updated).await?;

    let issue_block_link_id = updated
        .events()
        .iter()
        .next()
        .map(|event| event.key().0.to_owned())
        .expect("invalid event seq");
    Ok(IssueManagementContextEvent::IssueUnblocked {
        issue_block_link_id,
    })
}
