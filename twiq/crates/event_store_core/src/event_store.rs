use async_trait::async_trait;

use crate::{Event, EventId, EventStreamId, EventStreamSeq};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("unknown error : {0}")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[async_trait]
pub trait EventStore {
    async fn find_event(&self, event_id: EventId) -> Result<Option<Event>>;
    async fn find_event_ids(&self, after: Option<EventId>) -> Result<Vec<EventId>>;
    async fn find_event_stream(&self, event_stream_id: EventStreamId) -> Result<Vec<Event>>;
    async fn find_events(&self, after: Option<EventId>) -> Result<Vec<Event>>;
    async fn store(&self, current: Option<EventStreamSeq>, event_stream: Vec<Event>) -> Result<()>;
}
