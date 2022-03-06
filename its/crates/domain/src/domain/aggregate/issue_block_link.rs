mod error;
mod event;

use limited_date_time::Instant;

use crate::IssueBlocked;
use crate::{domain::entity::IssueBlockLink, IssueBlockLinkId, IssueId, Version};

pub use self::error::IssueBlockLinkAggregateError;
pub use self::event::IssueBlockLinkAggregateEvent;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueBlockLinkAggregate {
    events: Vec<IssueBlockLinkAggregateEvent>,
    issue_block_link: IssueBlockLink,
    version: Version,
}

pub type IssueBlockLinkAggregateResult =
    Result<(IssueBlockLinkAggregate, IssueBlockLinkAggregateEvent), IssueBlockLinkAggregateError>;

impl IssueBlockLinkAggregate {
    pub fn from_event(event: IssueBlocked) -> Result<Self, IssueBlockLinkAggregateError> {
        Self::block(
            event.at(),
            event.issue_id().clone(),
            event.blocked_issue_id().clone(),
        )
        .map(|issue_block_link| issue_block_link.truncate_events())
    }

    fn truncate_events(&self) -> Self {
        Self {
            events: vec![],
            issue_block_link: self.issue_block_link.clone(),
            version: self.version.clone(),
        }
    }

    pub fn block(
        at: Instant,
        issue_id: IssueId,
        blocked_issue_id: IssueId,
    ) -> Result<Self, IssueBlockLinkAggregateError> {
        let id = IssueBlockLinkId::new(issue_id, blocked_issue_id)
            .map_err(|_| IssueBlockLinkAggregateError::Block)?;
        let issue_block_link = IssueBlockLink::new(id.clone());
        let version = Version::from(1_u64);
        Ok(Self {
            events: vec![IssueBlockLinkAggregateEvent::Blocked(IssueBlocked {
                at,
                issue_block_link_id: id,
                version,
            })],
            issue_block_link,
            version,
        })
    }

    pub fn events(&self) -> &Vec<IssueBlockLinkAggregateEvent> {
        &self.events
    }

    pub fn unblock(&self) -> Result<Self, IssueBlockLinkAggregateError> {
        // TODO: check blocked
        let updated_issue_block_link = self.issue_block_link.unblock();
        let updated_version = self
            .version
            .next()
            .ok_or(IssueBlockLinkAggregateError::NoNextVersion)?;
        let mut updated_events = self.events.clone();
        updated_events.push(IssueBlockLinkAggregateEvent::Unblocked);
        Ok(Self {
            events: updated_events,
            issue_block_link: updated_issue_block_link,
            version: updated_version,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn from_event_test() {
        // TODO
    }
}
