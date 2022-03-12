use crate::{IssueBlockLinkId, IssueBlocked, IssueUnblocked, Version};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IssueBlockLinkAggregateEvent {
    Blocked(IssueBlocked),
    Unblocked(IssueUnblocked),
}

impl IssueBlockLinkAggregateEvent {
    pub fn key(&self) -> (&IssueBlockLinkId, Version) {
        match self {
            IssueBlockLinkAggregateEvent::Blocked(event) => event.key(),
            IssueBlockLinkAggregateEvent::Unblocked(event) => event.key(),
        }
    }
}

impl From<IssueBlocked> for IssueBlockLinkAggregateEvent {
    fn from(event: IssueBlocked) -> Self {
        Self::Blocked(event)
    }
}

impl From<IssueUnblocked> for IssueBlockLinkAggregateEvent {
    fn from(event: IssueUnblocked) -> Self {
        Self::Unblocked(event)
    }
}
