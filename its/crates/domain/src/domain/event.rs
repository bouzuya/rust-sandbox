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

#[derive(Debug)]
pub enum DomainEvent {
    Issue(IssueAggregateEvent),
    IssueBlockLink(IssueBlockLinkAggregateEvent),
}
