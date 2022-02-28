use async_trait::async_trait;
use domain::{
    aggregate::{IssueAggregate, IssueAggregateEvent},
    IssueId,
};
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    str::FromStr,
};
use use_case::{IssueRepository, IssueRepositoryError};

use crate::event_dto::EventDto;

#[derive(Debug, Default)]
pub struct FsIssueRepository {}

#[async_trait]
impl IssueRepository for FsIssueRepository {
    async fn find_by_id(
        &self,
        issue_id: &IssueId,
    ) -> Result<Option<IssueAggregate>, IssueRepositoryError> {
        let file_path = PathBuf::from_str("its.jsonl").map_err(|_| IssueRepositoryError::IO)?;
        if !file_path.exists() {
            return Ok(None);
        }

        let file = File::open(file_path.as_path()).map_err(|_| IssueRepositoryError::IO)?;
        let buf_reader = BufReader::new(file);
        let mut events: Vec<IssueAggregateEvent> = vec![];
        for line in buf_reader.lines() {
            let line = line.map_err(|_| IssueRepositoryError::IO)?;
            let dto = serde_json::from_str::<'_, EventDto>(line.as_str())
                .map_err(|_| IssueRepositoryError::IO)?;
            let event = IssueAggregateEvent::try_from(dto).map_err(|_| IssueRepositoryError::IO)?;
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
                .map_err(|_| IssueRepositoryError::IO)
        }
    }

    async fn last_created(&self) -> Result<Option<IssueAggregate>, IssueRepositoryError> {
        match self.max_issue_id()? {
            Some(issue_id) => self.find_by_id(&issue_id).await,
            None => Ok(None),
        }
    }

    async fn save(&self, event: IssueAggregateEvent) -> Result<(), IssueRepositoryError> {
        let file_path = PathBuf::from_str("its.jsonl").map_err(|_| IssueRepositoryError::IO)?;
        let mut events = self.events(file_path.as_path())?;

        events.push(event);

        let file = File::create(file_path.as_path()).map_err(|_| IssueRepositoryError::IO)?;
        let mut buf_writer = BufWriter::new(file);
        for event in events {
            let dto = EventDto::from(event);
            let line = serde_json::to_string(&dto).map_err(|_| IssueRepositoryError::IO)?;
            buf_writer
                .write(line.as_bytes())
                .map_err(|_| IssueRepositoryError::IO)?;
            buf_writer
                .write("\n".as_bytes())
                .map_err(|_| IssueRepositoryError::IO)?;
        }
        Ok(())
    }
}

impl FsIssueRepository {
    fn events(&self, file_path: &Path) -> Result<Vec<IssueAggregateEvent>, IssueRepositoryError> {
        Ok(if file_path.exists() {
            let file = File::open(file_path).map_err(|_| IssueRepositoryError::IO)?;
            let buf_reader = BufReader::new(file);
            let mut events: Vec<IssueAggregateEvent> = vec![];
            for line in buf_reader.lines() {
                let line = line.map_err(|_| IssueRepositoryError::IO)?;
                let dto = serde_json::from_str::<'_, EventDto>(line.as_str())
                    .map_err(|_| IssueRepositoryError::IO)?;
                let event =
                    IssueAggregateEvent::try_from(dto).map_err(|_| IssueRepositoryError::IO)?;
                events.push(event);
            }
            events
        } else {
            vec![]
        })
    }

    fn max_issue_id(&self) -> Result<Option<IssueId>, IssueRepositoryError> {
        let file_path = PathBuf::from_str("its.jsonl").map_err(|_| IssueRepositoryError::IO)?;
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
        Ok(max)
    }
}
