use crate::{IssueBlocked, IssueUnblocked};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IssueBlockLinkAggregateEvent {
    Blocked(IssueBlocked),
    Unblocked(IssueUnblocked),
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
