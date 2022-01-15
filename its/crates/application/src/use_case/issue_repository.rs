use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use domain::{
    aggregate::{IssueAggregate, IssueAggregateEvent},
    IssueId, IssueNumber,
};
use thiserror::Error;

use crate::use_case::event_dto::EventDto;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("IO")]
    IO,
}

#[derive(Debug, Default)]
pub struct IssueRepository {}

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
                IssueAggregateEvent::Created(event) => event.issue_id() == issue_id,
                IssueAggregateEvent::CreatedV2(event) => event.issue_id() == issue_id,
                IssueAggregateEvent::Finished(event) => event.issue_id() == issue_id,
                IssueAggregateEvent::Updated(event) => event.issue_id() == issue_id,
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
        let mut events = self.events(file_path.as_path())?;

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

    pub fn next_issue_number(&self) -> Result<IssueNumber, RepositoryError> {
        let file_path = PathBuf::from_str("its.jsonl").map_err(|_| RepositoryError::IO)?;
        let events = self.events(file_path.as_path())?;
        let mut max: Option<IssueId> = None;
        for event in events {
            match event {
                IssueAggregateEvent::Created(event) => {
                    max = Some(
                        max.unwrap_or_else(|| event.issue_id().clone())
                            .max(event.issue_id().clone()),
                    )
                }
                IssueAggregateEvent::CreatedV2(event) => {
                    max = Some(
                        max.unwrap_or_else(|| event.issue_id().clone())
                            .max(event.issue_id().clone()),
                    )
                }
                IssueAggregateEvent::Finished(_) => {}
                IssueAggregateEvent::Updated(_) => {}
            }
        }
        Ok(max
            .map(|id| id.issue_number().next_number())
            .unwrap_or_else(IssueNumber::start_number))
    }

    fn events(&self, file_path: &Path) -> Result<Vec<IssueAggregateEvent>, RepositoryError> {
        Ok(if file_path.exists() {
            let file = File::open(file_path).map_err(|_| RepositoryError::IO)?;
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
        })
    }
}
