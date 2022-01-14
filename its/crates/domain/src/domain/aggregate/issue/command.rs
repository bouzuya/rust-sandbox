use limited_date_time::Instant;

use super::IssueAggregate;
use crate::{
    domain::{IssueNumber, IssueTitle},
    IssueDue,
};

#[derive(Debug)]
pub enum IssueAggregateCommand {
    Create(IssueAggregateCreateIssue),
    Finish(IssueAggregateFinishIssue),
}

#[derive(Debug)]
pub struct IssueAggregateCreateIssue {
    pub issue_number: IssueNumber,
    pub issue_title: IssueTitle,
    pub issue_due: Option<IssueDue>,
    pub at: Instant,
}

#[derive(Debug)]
pub struct IssueAggregateFinishIssue {
    pub issue: IssueAggregate,
    pub at: Instant,
}
