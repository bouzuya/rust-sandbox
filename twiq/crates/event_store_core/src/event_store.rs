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
    async fn find_event_ids(&self) -> Result<Vec<EventId>>;
    async fn find_event_ids_by_event_id_after(&self, event_id: EventId) -> Result<Vec<EventId>>;
    async fn find_events_by_event_id_after(&self, event_id: EventId) -> Result<Vec<Event>>;

    // = find_event_stream_by_id
    async fn find_events_by_event_stream_id(
        &self,
        event_stream_id: EventStreamId,
    ) -> Result<Vec<Event>>;

    // = store_event_stream
    async fn store(&self, current: Option<EventStreamSeq>, events: Vec<Event>) -> Result<()>;
}
