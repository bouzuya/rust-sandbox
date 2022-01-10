use crate::domain::event::{IssueCreated, IssueFinished};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IssueAggregateEvent {
    Created(IssueCreated),
    Finished(IssueFinished),
}
