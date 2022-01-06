use limited_date_time::Instant;

use super::IssueAggregate;

#[derive(Clone, Debug)]
pub enum IssueAggregateEvent {
    Created(IssueCreated),
}

#[derive(Clone, Debug)]
pub struct IssueCreated {
    pub at: Instant,
    pub issue: IssueAggregate,
}
