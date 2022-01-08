use limited_date_time::Instant;

use crate::{IssueId, IssueTitle, Version};

#[derive(Clone, Debug)]
pub enum IssueAggregateEvent {
    Created(IssueCreated),
    Finished(IssueFinished),
}

#[derive(Clone, Debug)]
pub struct IssueCreated {
    pub at: Instant,
    pub issue_id: IssueId,
    pub issue_title: IssueTitle,
    pub version: Version,
}

#[derive(Clone, Debug)]
pub struct IssueFinished {
    pub at: Instant,
    pub issue_id: IssueId,
    pub version: Version,
}
