use async_trait::async_trait;

use crate::{Event, EventId, EventStreamId, EventStreamSeq};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown error : {0}")]
    Unknown(String),
}

#[async_trait]
pub trait EventStore {
    async fn find_events_by_event_id_after(&self, event_id: EventId) -> Result<Vec<Event>, Error>;

    // = find_event_stream_by_id
    async fn find_events_by_event_stream_id(
        &self,
        event_stream_id: EventStreamId,
    ) -> Result<Vec<Event>, Error>;

    // = store_event_stream
    async fn store(&self, current: Option<EventStreamSeq>, events: Vec<Event>)
        -> Result<(), Error>;
}
