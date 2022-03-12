use limited_date_time::Instant;

use super::IssueAggregate;
use crate::IssueDue;

#[derive(Debug)]
pub enum IssueAggregateCommand {
    Finish(IssueAggregateFinishIssue),
    Update(IssueAggregateUpdateIssue),
}

#[derive(Debug)]
pub struct IssueAggregateFinishIssue {
    pub issue: IssueAggregate,
    pub at: Instant,
}

#[derive(Debug)]
pub struct IssueAggregateUpdateIssue {
    pub issue: IssueAggregate,
    pub issue_due: Option<IssueDue>,
    pub at: Instant,
}
