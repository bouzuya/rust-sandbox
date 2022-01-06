use limited_date_time::Instant;

use crate::entity::{IssueNumber, IssueTitle};

#[derive(Debug)]
pub enum IssueAggregateCommand {
    Create(IssueAggregateCreateIssue),
}

#[derive(Debug)]
pub struct IssueAggregateCreateIssue {
    pub issue_number: IssueNumber,
    pub issue_title: IssueTitle,
    pub at: Instant,
}
