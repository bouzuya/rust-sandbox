mod command;
mod error;
mod event;
mod transaction;

pub use self::command::*;
pub use self::error::*;
pub use self::event::*;
use self::transaction::*;
use crate::domain::entity::Issue;
use crate::IssueId;
use crate::Version;

#[derive(Clone, Debug)]
pub struct IssueAggregate {
    issue: Issue,
    version: Version,
}

impl IssueAggregate {
    pub fn from_events(events: &[IssueAggregateEvent]) -> Result<Self, IssueAggregateError> {
        if let Some(IssueAggregateEvent::Created(IssueCreated { at: _, issue: i })) = events.first()
        {
            let mut issue = i.clone();
            for event in events.iter().skip(1) {
                match event {
                    IssueAggregateEvent::Created(_) => {
                        return Err(IssueAggregateError::Unknown);
                    }
                    IssueAggregateEvent::Finished(IssueFinished {
                        at: _,
                        issue_id,
                        version,
                    }) => {
                        if issue.issue.id() != issue_id {
                            return Err(IssueAggregateError::Unknown);
                        }
                        if issue.version.next() != Some(*version) {
                            return Err(IssueAggregateError::Unknown);
                        }

                        issue = IssueAggregate {
                            issue: issue
                                .issue
                                .finish()
                                .map_err(|_| IssueAggregateError::Unknown)?,
                            version: *version,
                        }
                    }
                }
            }
            Ok(issue)
        } else {
            Err(IssueAggregateError::Unknown)
        }
    }

    pub fn transaction(
        command: IssueAggregateCommand,
    ) -> Result<(IssueAggregate, IssueAggregateEvent), IssueAggregateError> {
        match command {
            IssueAggregateCommand::Create(command) => create_issue(command),
            IssueAggregateCommand::Finish(command) => finish_issue(command),
        }
    }

    pub fn id(&self) -> &IssueId {
        self.issue.id()
    }

    pub fn issue(&self) -> &Issue {
        &self.issue
    }
}
