mod event_dto;
mod issue_repository;

pub use self::issue_repository::*;

use domain::{
    aggregate::{
        IssueAggregate, IssueAggregateCommand, IssueAggregateCreateIssue, IssueAggregateError,
        IssueAggregateEvent, IssueAggregateFinishIssue,
    },
    IssueCreatedV2, IssueDue, IssueFinished, IssueId, IssueNumber, IssueTitle,
};
use limited_date_time::Instant;
use thiserror::Error;

#[derive(Debug)]
pub enum IssueManagementContextCommand {
    CreateIssue(CreateIssue),
    FinishIssue(FinishIssue),
}

#[derive(Debug)]
pub struct CreateIssue {
    pub issue_title: IssueTitle,
    pub issue_due: Option<IssueDue>,
}

#[derive(Debug)]
pub struct FinishIssue {
    pub issue_id: IssueId,
}

#[derive(Debug)]
pub enum IssueManagementContextEvent {
    IssueCreated(IssueCreatedV2),
    IssueFinished(IssueFinished),
}

#[derive(Debug, Error)]
pub enum IssueManagementContextError {
    #[error("IssueAggregate")]
    IssueAggregate(IssueAggregateError),
    #[error("Unknown")]
    Unknown,
}

pub fn issue_management_context_use_case(
    command: IssueManagementContextCommand,
) -> Result<IssueManagementContextEvent, IssueManagementContextError> {
    match command {
        IssueManagementContextCommand::CreateIssue(command) => {
            let event = create_issue_use_case(command)?;
            Ok(IssueManagementContextEvent::IssueCreated(event))
        }
        IssueManagementContextCommand::FinishIssue(command) => {
            let event = finish_issue_use_case(command)?;
            Ok(IssueManagementContextEvent::IssueFinished(event))
        }
    }
}

pub fn create_issue_use_case(
    command: CreateIssue,
) -> Result<IssueCreatedV2, IssueManagementContextError> {
    let issue_repository = IssueRepository::default(); // TODO: dependency

    // io
    let issue_number = issue_repository
        .last_created()
        .map_err(|_| IssueManagementContextError::Unknown)?
        .map(|issue| issue.id().issue_number())
        .unwrap_or_else(IssueNumber::start_number)
        .next_number();
    let at = Instant::now();

    // pure
    let (_, event) =
        IssueAggregate::transaction(IssueAggregateCommand::Create(IssueAggregateCreateIssue {
            issue_number,
            issue_title: command.issue_title,
            issue_due: command.issue_due,
            at,
        }))
        .map_err(IssueManagementContextError::IssueAggregate)?;

    // io
    issue_repository
        .save(event.clone())
        .map_err(|_| IssueManagementContextError::Unknown)?;

    if let IssueAggregateEvent::CreatedV2(event) = event {
        Ok(event)
    } else {
        unreachable!()
    }
}

pub fn finish_issue_use_case(
    command: FinishIssue,
) -> Result<IssueFinished, IssueManagementContextError> {
    let issue_repository = IssueRepository::default(); // TODO: dependency

    // io
    let issue = issue_repository
        .find_by_id(&command.issue_id)
        .map_err(|_| IssueManagementContextError::Unknown)?;
    // TODO: fix error
    let issue = issue.ok_or(IssueManagementContextError::Unknown)?;
    let at = Instant::now();

    // pure
    let (_, event) =
        IssueAggregate::transaction(IssueAggregateCommand::Finish(IssueAggregateFinishIssue {
            issue,
            at,
        }))
        .map_err(IssueManagementContextError::IssueAggregate)?;

    // io
    issue_repository
        .save(event.clone())
        .map_err(|_| IssueManagementContextError::Unknown)?;

    if let IssueAggregateEvent::Finished(event) = event {
        Ok(event)
    } else {
        unreachable!()
    }
}
