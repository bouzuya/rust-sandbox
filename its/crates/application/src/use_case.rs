use entity::{
    IssueAggregate, IssueAggregateCommand, IssueAggregateCreateIssue, IssueAggregateError,
    IssueAggregateEvent, IssueAggregateFinishIssue, IssueCreated, IssueFinished, IssueId,
    IssueNumber, IssueTitle,
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
}

#[derive(Debug)]
pub struct FinishIssue {
    pub issue_id: IssueId,
}

#[derive(Debug)]
pub enum IssueManagementContextEvent {
    IssueCreated(IssueCreated),
    IssueFinished(IssueFinished),
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
    issues: Vec<IssueAggregate>,
}

impl IssueRepository {
    // TODO: Result
    pub fn find_by_id(&self, issue_id: IssueId) -> Option<IssueAggregate> {
        self.issues
            .iter()
            .find(|issue| issue.id() == &issue_id)
            .cloned()
    }

    pub fn next_issue_number(&self) -> IssueNumber {
        if let Some(last_issue) = self.issues.last() {
            last_issue.issue().number().next_number()
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
        IssueManagementContextCommand::FinishIssue(command) => {
            let event = finish_issue_use_case(command)?;
            Ok(IssueManagementContextEvent::IssueFinished(event))
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
    let (_, event) =
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

pub fn finish_issue_use_case(
    command: FinishIssue,
) -> Result<IssueFinished, IssueManagementContextError> {
    let issue_repository = IssueRepository::default(); // TODO: dependency

    // io
    let issue = issue_repository.find_by_id(command.issue_id);
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
    // TODO: save issue

    if let IssueAggregateEvent::Finished(event) = event {
        Ok(event)
    } else {
        unreachable!()
    }
}
