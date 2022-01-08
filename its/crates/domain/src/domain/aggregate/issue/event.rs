use limited_date_time::Instant;

use crate::{IssueId, Version};

use super::IssueAggregate;

#[derive(Clone, Debug)]
pub enum IssueAggregateEvent {
    Created(IssueCreated),
    Finished(IssueFinished),
}

#[derive(Clone, Debug)]
pub struct IssueCreated {
    pub at: Instant,
    // TODO: don't use aggregate
    pub issue: IssueAggregate,
}

#[derive(Clone, Debug)]
pub struct IssueFinished {
    pub at: Instant,
    pub issue_id: IssueId,
    pub version: Version,
}
