use limited_date_time::Instant;

use crate::{IssueId, IssueTitle, Version};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IssueAggregateEvent {
    Created(IssueCreated),
    Finished(IssueFinished),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueCreated {
    pub at: Instant,
    pub issue_id: IssueId,
    pub issue_title: IssueTitle,
    pub version: Version,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueFinished {
    pub at: Instant,
    pub issue_id: IssueId,
    pub version: Version,
}
