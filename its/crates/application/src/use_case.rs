mod event_dto;

use self::event_dto::*;

use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
    str::FromStr,
};

use domain::{
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

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("IO")]
    IO,
}

#[derive(Debug, Default)]
pub struct IssueRepository {
    issues: Vec<IssueAggregate>,
}

impl IssueRepository {
    pub fn find_by_id(
        &self,
        issue_id: &IssueId,
    ) -> Result<Option<IssueAggregate>, RepositoryError> {
        let file_path = PathBuf::from_str("its.jsonl").map_err(|_| RepositoryError::IO)?;
        if !file_path.exists() {
            return Ok(None);
        }

        let file = File::open(file_path.as_path()).map_err(|_| RepositoryError::IO)?;
        let buf_reader = BufReader::new(file);
        let mut events: Vec<IssueAggregateEvent> = vec![];
        for line in buf_reader.lines() {
            let line = line.map_err(|_| RepositoryError::IO)?;
            let dto = serde_json::from_str::<'_, EventDto>(line.as_str())
                .map_err(|_| RepositoryError::IO)?;
            let event = IssueAggregateEvent::try_from(dto).map_err(|_| RepositoryError::IO)?;
            events.push(event);
        }

        let filtered = events
            .into_iter()
            .filter(|e| match e {
                IssueAggregateEvent::Created(IssueCreated {
                    at: _,
                    issue_id: id,
                    issue_title: _,
                    version: _,
                }) => id == issue_id,
                IssueAggregateEvent::Finished(IssueFinished {
                    at: _,
                    issue_id: id,
                    version: _,
                }) => id == issue_id,
            })
            .collect::<Vec<IssueAggregateEvent>>();

        if filtered.is_empty() {
            Ok(None)
        } else {
            IssueAggregate::from_events(&filtered)
                .map(Some)
                .map_err(|_| RepositoryError::IO)
        }
    }

    pub fn save(&self, event: IssueAggregateEvent) -> Result<(), RepositoryError> {
        let file_path = PathBuf::from_str("its.jsonl").map_err(|_| RepositoryError::IO)?;
        let mut events = if file_path.exists() {
            let file = File::open(file_path.as_path()).map_err(|_| RepositoryError::IO)?;
            let buf_reader = BufReader::new(file);
            let mut events: Vec<IssueAggregateEvent> = vec![];
            for line in buf_reader.lines() {
                let line = line.map_err(|_| RepositoryError::IO)?;
                let dto = serde_json::from_str::<'_, EventDto>(line.as_str())
                    .map_err(|_| RepositoryError::IO)?;
                let event = IssueAggregateEvent::try_from(dto).map_err(|_| RepositoryError::IO)?;
                events.push(event);
            }
            events
        } else {
            vec![]
        };

        events.push(event);

        let file = File::create(file_path.as_path()).map_err(|_| RepositoryError::IO)?;
        let mut buf_writer = BufWriter::new(file);
        for event in events {
            let dto = EventDto::from(event);
            let line = serde_json::to_string(&dto).map_err(|_| RepositoryError::IO)?;
            buf_writer
                .write(line.as_bytes())
                .map_err(|_| RepositoryError::IO)?;
            buf_writer
                .write("\n".as_bytes())
                .map_err(|_| RepositoryError::IO)?;
        }
        Ok(())
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
    issue_repository
        .save(event.clone())
        .map_err(|_| IssueManagementContextError::Unknown)?;

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
