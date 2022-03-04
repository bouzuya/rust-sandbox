mod issue_blocked;
mod issue_created;
mod issue_created_v2;
mod issue_finished;
mod issue_updated;

use crate::aggregate::IssueAggregateEvent;
use crate::aggregate::IssueBlockLinkAggregateEvent;

pub use self::issue_blocked::*;
pub use self::issue_created::*;
pub use self::issue_created_v2::*;
pub use self::issue_finished::*;
pub use self::issue_updated::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DomainEvent {
    Issue(IssueAggregateEvent),
    IssueBlockLink(IssueBlockLinkAggregateEvent),
}

impl From<IssueAggregateEvent> for DomainEvent {
    fn from(event: IssueAggregateEvent) -> Self {
        Self::Issue(event)
    }
}

impl From<IssueBlockLinkAggregateEvent> for DomainEvent {
    fn from(event: IssueBlockLinkAggregateEvent) -> Self {
        Self::IssueBlockLink(event)
    }
}

impl DomainEvent {
    pub fn issue(self) -> Option<IssueAggregateEvent> {
        if let Self::Issue(event) = self {
            Some(event)
        } else {
            None
        }
    }

    pub fn issue_block_link(self) -> Option<IssueBlockLinkAggregateEvent> {
        if let Self::IssueBlockLink(event) = self {
            Some(event)
        } else {
            None
        }
    }
}
