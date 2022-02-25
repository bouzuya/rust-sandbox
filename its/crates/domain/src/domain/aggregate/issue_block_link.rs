mod error;

use crate::{domain::entity::IssueBlockLink, IssueBlockLinkId, IssueId, Version};

pub use self::error::IssueBlockLinkAggregateError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueBlockLinkAggregate {
    issue_block_link: IssueBlockLink,
    version: Version,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IssueBlockLinkAggregateEvent {
    // TODO: Id
    Blocked,
    // TODO: Id
    Unblocked,
}

pub type IssueBlockLinkAggregateResult =
    Result<(IssueBlockLinkAggregate, IssueBlockLinkAggregateEvent), IssueBlockLinkAggregateError>;

impl IssueBlockLinkAggregate {
    pub fn block(issue_id: IssueId, blocked_issue_id: IssueId) -> IssueBlockLinkAggregateResult {
        let id = IssueBlockLinkId::new(issue_id, blocked_issue_id)
            .map_err(|_| IssueBlockLinkAggregateError::Block)?;
        let issue_block_link = IssueBlockLink::new(id);
        Ok((
            Self {
                issue_block_link,
                version: Version::from(1_u64),
            },
            IssueBlockLinkAggregateEvent::Blocked,
        ))
    }

    pub fn unblock(&self) -> IssueBlockLinkAggregateResult {
        // TODO: check blocked
        let updated_issue_block_link = self.issue_block_link.unblock();
        let updated_version = self
            .version
            .next()
            .ok_or(IssueBlockLinkAggregateError::NoNextVersion)?;
        Ok((
            Self {
                issue_block_link: updated_issue_block_link,
                version: updated_version,
            },
            IssueBlockLinkAggregateEvent::Unblocked,
        ))
    }
}
