mod command;
mod error;
mod event;
mod transaction;

pub use self::command::*;
pub use self::error::*;
pub use self::event::*;
use self::transaction::*;
use crate::entity::version::Version;
use crate::entity::Issue;

#[derive(Clone, Debug)]
pub struct IssueAggregate {
    issue: Issue,
    version: Version,
}

impl IssueAggregate {
    pub fn from_events(events: &[IssueAggregateEvent]) -> Result<Self, IssueAggregateError> {
        let mut issue = None;
        for event in events {
            match event {
                IssueAggregateEvent::Created(IssueCreated { at: _, issue: i }) => {
                    issue = Some(i);
                }
            }
        }
        Ok(issue.ok_or(IssueAggregateError::Unknown)?.clone())
    }

    pub fn transaction(
        command: IssueAggregateCommand,
    ) -> Result<IssueAggregateEvent, IssueAggregateError> {
        match command {
            IssueAggregateCommand::Create(command) => create_issue(command),
        }
    }
}
