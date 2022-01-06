use crate::entity::{
    Issue, IssueAggregate, IssueAggregateCommand, IssueAggregateCreateIssue, IssueAggregateError,
    IssueAggregateEvent, IssueCreated, IssueNumber, IssueTitle,
};
use limited_date_time::Instant;
use thiserror::Error;

#[derive(Debug)]
pub enum IssueManagementContextCommand {
    CreateIssue(CreateIssue),
}

#[derive(Debug)]
pub struct CreateIssue {
    pub issue_title: IssueTitle,
}

#[derive(Debug)]
pub enum IssueManagementContextEvent {
    IssueCreated(IssueCreated),
}

#[derive(Debug, Error)]
pub enum IssueManagementContextError {
    #[error("IssueAggregate")]
    IssueAggregate(IssueAggregateError),
    #[error("Unknown")]
    Unknown,
}

#[derive(Debug, Default)]
pub struct IssueRepository {
    issues: Vec<Issue>,
}

impl IssueRepository {
    pub fn next_issue_number(&self) -> IssueNumber {
        if let Some(last_issue) = self.issues.last() {
            last_issue.number().next_number()
        } else {
            IssueNumber::start_number()
        }
    }
}

pub fn issue_management_context_use_case(
    command: IssueManagementContextCommand,
) -> Result<IssueManagementContextEvent, IssueManagementContextError> {
    match command {
        IssueManagementContextCommand::CreateIssue(command) => {
            let event = create_issue_use_case(command)?;
            Ok(IssueManagementContextEvent::IssueCreated(event))
        }
    }
}

pub fn create_issue_use_case(
    command: CreateIssue,
) -> Result<IssueCreated, IssueManagementContextError> {
    let issue_repository = IssueRepository::default(); // TODO: dependency

    // io
    let issue_number = issue_repository.next_issue_number();
    let at = Instant::now();

    // pure
    let event =
        IssueAggregate::transaction(IssueAggregateCommand::Create(IssueAggregateCreateIssue {
            issue_number,
            issue_title: command.issue_title,
            at,
        }))
        .map_err(IssueManagementContextError::IssueAggregate)?;

    // io
    // TODO: save issue

    if let IssueAggregateEvent::Created(event) = event {
        Ok(event)
    } else {
        unreachable!()
    }
}
