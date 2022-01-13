use crate::domain::event::{IssueCreated, IssueCreatedV2, IssueFinished};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IssueAggregateEvent {
    Created(IssueCreated),
    CreatedV2(IssueCreatedV2),
    Finished(IssueFinished),
}
