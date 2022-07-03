use domain::{
    aggregate::IssueBlockLinkAggregateError, DomainEvent, IssueBlockLinkId, IssueId,
    ParseIssueBlockLinkError,
};
use limited_date_time::Instant;

use crate::{
    HasIssueBlockLinkRepository, HasIssueRepository, IssueBlockLinkRepository,
    IssueBlockLinkRepositoryError, IssueManagementContextEvent, IssueRepository,
    IssueRepositoryError,
};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("issue block link aggregate {0}")]
    IssueBlockLinkAggregate(#[from] IssueBlockLinkAggregateError),
    #[error("issue block link repository {0}")]
    IssueBlockLinkRepository(#[from] IssueBlockLinkRepositoryError),
    #[error("issue not found {0}")]
    IssueNotFound(IssueId),
    #[error("issue repository {0}")]
    IssueRepository(#[from] IssueRepositoryError),
    #[error("parse issue block link id {0}")]
    ParseIssueBlockLink(#[from] ParseIssueBlockLinkError),
}

#[derive(Debug, Eq, PartialEq)]
pub struct BlockIssue {
    pub issue_id: IssueId,
    pub blocked_issue_id: IssueId,
}

pub async fn block_issue<C: HasIssueBlockLinkRepository + HasIssueRepository + ?Sized>(
    context: &C,
    BlockIssue {
        issue_id,
        blocked_issue_id,
    }: BlockIssue,
) -> Result<Vec<IssueManagementContextEvent>, Error> {
    // io
    let at = Instant::now();
    let issue_block_link_id = IssueBlockLinkId::new(issue_id.clone(), blocked_issue_id.clone())?;
    let issue_block_link = match context
        .issue_block_link_repository()
        .find_by_id(&issue_block_link_id)
        .await?
    {
        Some(issue_block_link) => issue_block_link.block(at)?,
        None => {
            // io
            let issue = context
                .issue_repository()
                .find_by_id(&issue_id)
                .await?
                .ok_or(Error::IssueNotFound(issue_id))?;
            let blocked_issue = context
                .issue_repository()
                .find_by_id(&blocked_issue_id)
                .await?
                .ok_or(Error::IssueNotFound(blocked_issue_id))?;

            // pure
            issue.block(blocked_issue, at)?
        }
    };

    // io
    context
        .issue_block_link_repository()
        .save(&issue_block_link)
        .await?;

    Ok(issue_block_link
        .events()
        .iter()
        .cloned()
        .map(DomainEvent::from)
        .map(IssueManagementContextEvent::from)
        .collect::<Vec<IssueManagementContextEvent>>())
}
