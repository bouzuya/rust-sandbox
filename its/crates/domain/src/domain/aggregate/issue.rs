mod command;
mod error;
mod event;
mod transaction;

pub use self::command::*;
pub use self::error::*;
pub use self::event::*;
use self::transaction::*;
use crate::IssueCreatedV2;
use crate::IssueUpdated;
use crate::{
    domain::{entity::Issue, event::IssueFinished},
    IssueId, Version,
};

use super::IssueBlockLinkAggregate;
use super::IssueBlockLinkAggregateResult;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueAggregate {
    issue: Issue,
    version: Version,
}

impl IssueAggregate {
    pub fn from_events(events: &[IssueAggregateEvent]) -> Result<Self, IssueAggregateError> {
        let first_event = match events.first() {
            Some(event) => match event {
                IssueAggregateEvent::Created(event) => Ok(IssueCreatedV2::from_v1(event.clone())),
                IssueAggregateEvent::CreatedV2(event) => Ok(event.clone()),
                IssueAggregateEvent::Finished(_) => Err(IssueAggregateError::InvalidEventSequence),
                IssueAggregateEvent::Updated(_) => Err(IssueAggregateError::InvalidEventSequence),
            },
            None => Err(IssueAggregateError::InvalidEventSequence),
        }?;
        let version = first_event.version;
        let issue = Issue::from_event(first_event);
        let mut issue = IssueAggregate { issue, version };
        for event in events.iter().skip(1) {
            match event {
                IssueAggregateEvent::Created(_) => {
                    return Err(IssueAggregateError::InvalidEventSequence);
                }
                IssueAggregateEvent::CreatedV2(_) => {
                    return Err(IssueAggregateError::InvalidEventSequence);
                }
                IssueAggregateEvent::Finished(IssueFinished {
                    at: _,
                    issue_id,
                    version,
                }) => {
                    if issue.issue.id() != issue_id {
                        return Err(IssueAggregateError::InvalidEventSequence);
                    }
                    if issue.version.next() != Some(*version) {
                        return Err(IssueAggregateError::InvalidEventSequence);
                    }

                    issue = IssueAggregate {
                        issue: issue
                            .issue
                            .finish()
                            .map_err(|_| IssueAggregateError::InvalidEventSequence)?,
                        version: *version,
                    }
                }
                IssueAggregateEvent::Updated(IssueUpdated {
                    at: _,
                    issue_id,
                    issue_due,
                    version,
                }) => {
                    if issue.issue.id() != issue_id {
                        return Err(IssueAggregateError::InvalidEventSequence);
                    }
                    if issue.version.next() != Some(*version) {
                        return Err(IssueAggregateError::InvalidEventSequence);
                    }

                    issue = IssueAggregate {
                        issue: issue.issue.change_due(*issue_due),
                        version: *version,
                    }
                }
            }
        }
        Ok(issue)
    }

    pub fn transaction(
        command: IssueAggregateCommand,
    ) -> Result<(IssueAggregate, IssueAggregateEvent), IssueAggregateError> {
        match command {
            IssueAggregateCommand::Create(command) => create_issue(command),
            IssueAggregateCommand::Finish(command) => finish_issue(command),
            IssueAggregateCommand::Update(command) => update_issue(command),
        }
    }

    pub fn id(&self) -> &IssueId {
        self.issue.id()
    }

    pub fn issue(&self) -> &Issue {
        &self.issue
    }

    pub fn block(&self, blocked_issue: IssueAggregate) -> IssueBlockLinkAggregateResult {
        IssueBlockLinkAggregate::block(self.id().clone(), blocked_issue.id().clone())
    }
}
