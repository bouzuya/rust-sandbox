use crate::entity::{version::Version, Issue, IssueId};

use super::{
    IssueAggregate, IssueAggregateCreateIssue, IssueAggregateError, IssueAggregateEvent,
    IssueCreated,
};

pub fn create_issue(
    command: IssueAggregateCreateIssue,
) -> Result<IssueAggregateEvent, IssueAggregateError> {
    let issue_id = IssueId::new(command.issue_number);
    let issue = Issue::new(issue_id, command.issue_title);
    let event = IssueAggregateEvent::Created(IssueCreated {
        at: command.at,
        issue: IssueAggregate {
            issue,
            version: Version::from(1_u64),
        },
    });
    Ok(event)
}
