mod error;
mod event;

use limited_date_time::Instant;

use crate::IssueBlocked;
use crate::{domain::entity::IssueBlockLink, IssueBlockLinkId, IssueId, Version};

pub use self::error::IssueBlockLinkAggregateError;
pub use self::event::IssueBlockLinkAggregateEvent;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueBlockLinkAggregate {
    issue_block_link: IssueBlockLink,
    version: Version,
}

pub type IssueBlockLinkAggregateResult =
    Result<(IssueBlockLinkAggregate, IssueBlockLinkAggregateEvent), IssueBlockLinkAggregateError>;

impl IssueBlockLinkAggregate {
    pub fn from_event(event: IssueBlocked) -> Result<Self, IssueBlockLinkAggregateError> {
        Self::block(event.issue_id().clone(), event.blocked_issue_id().clone())
            .map(|(issue_block_link, _)| issue_block_link)
    }

    pub fn block(issue_id: IssueId, blocked_issue_id: IssueId) -> IssueBlockLinkAggregateResult {
        let id = IssueBlockLinkId::new(issue_id, blocked_issue_id)
            .map_err(|_| IssueBlockLinkAggregateError::Block)?;
        let issue_block_link = IssueBlockLink::new(id.clone());
        let version = Version::from(1_u64);
        Ok((
            Self {
                issue_block_link,
                version,
            },
            IssueBlockLinkAggregateEvent::Blocked(IssueBlocked {
                at: Instant::now(), // FIXME
                issue_block_link_id: id,
                version,
            }),
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

#[cfg(test)]
mod tests {
    #[test]
    fn from_event_test() {
        // TODO
    }
}
