use crate::{
    entity::{version::Version, Issue, IssueId},
    IssueAggregateFinishIssue, IssueFinished,
};

use super::{
    IssueAggregate, IssueAggregateCreateIssue, IssueAggregateError, IssueAggregateEvent,
    IssueCreated,
};

pub fn create_issue(
    command: IssueAggregateCreateIssue,
) -> Result<(IssueAggregate, IssueAggregateEvent), IssueAggregateError> {
    let issue_id = IssueId::new(command.issue_number);
    let issue = Issue::new(issue_id, command.issue_title);
    let issue = IssueAggregate {
        issue,
        version: Version::from(1_u64),
    };
    let event = IssueAggregateEvent::Created(IssueCreated {
        at: command.at,
        issue: issue.clone(),
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
        issue_id: aggregate.issue.id,
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
