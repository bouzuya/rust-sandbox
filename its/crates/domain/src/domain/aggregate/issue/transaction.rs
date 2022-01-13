use crate::{
    domain::{entity::Issue, event::IssueFinished, IssueId},
    IssueAggregateFinishIssue, IssueCreatedV2, Version,
};

use super::{IssueAggregate, IssueAggregateCreateIssue, IssueAggregateError, IssueAggregateEvent};

pub fn create_issue(
    command: IssueAggregateCreateIssue,
) -> Result<(IssueAggregate, IssueAggregateEvent), IssueAggregateError> {
    let issue_id = IssueId::new(command.issue_number);
    let issue_title = command.issue_title;
    let issue_due = command.issue_due;
    let issue = Issue::new(issue_id.clone(), issue_title.clone(), issue_due);
    let version = Version::from(1_u64);
    let issue = IssueAggregate { issue, version };
    let event = IssueAggregateEvent::CreatedV2(IssueCreatedV2 {
        at: command.at,
        issue_id,
        issue_title,
        issue_due,
        version,
    });
    Ok((issue, event))
}

pub fn finish_issue(
    command: IssueAggregateFinishIssue,
) -> Result<(IssueAggregate, IssueAggregateEvent), IssueAggregateError> {
    let aggregate = command.issue;
    let updated_issue = aggregate
        .issue
        .finish()
        .map_err(|_| IssueAggregateError::Unknown)?;
    let updated_version = aggregate
        .version
        .next()
        .ok_or(IssueAggregateError::Unknown)?;
    let event = IssueAggregateEvent::Finished(IssueFinished {
        at: command.at,
        issue_id: aggregate.id().clone(),
        version: updated_version,
    });
    Ok((
        IssueAggregate {
            issue: updated_issue,
            version: updated_version,
        },
        event,
    ))
}
