use crate::{domain::event::IssueFinished, IssueUpdated};

use super::{
    IssueAggregate, IssueAggregateError, IssueAggregateEvent, IssueAggregateFinishIssue,
    IssueAggregateUpdateIssue,
};

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
    let event = IssueFinished {
        at: command.at,
        issue_id: aggregate.id().clone(),
        version: updated_version,
    }
    .into();
    Ok((
        IssueAggregate {
            issue: updated_issue,
            version: updated_version,
        },
        event,
    ))
}

pub fn update_issue(
    command: IssueAggregateUpdateIssue,
) -> Result<(IssueAggregate, IssueAggregateEvent), IssueAggregateError> {
    let aggregate = command.issue;
    let updated_issue = aggregate.issue.change_due(command.issue_due);
    let updated_version = aggregate
        .version
        .next()
        .ok_or(IssueAggregateError::Unknown)?;
    let event = IssueUpdated {
        at: command.at,
        issue_id: aggregate.id().clone(),
        issue_due: updated_issue.due(),
        version: updated_version,
    }
    .into();
    Ok((
        IssueAggregate {
            issue: updated_issue,
            version: updated_version,
        },
        event,
    ))
}
