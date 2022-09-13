use async_trait::async_trait;
use event_store_core::{event::Event, event_id::EventId};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("unknown {0}")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[async_trait]
pub trait EventStore {
    async fn find_event(&self, event_id: EventId) -> Result<Option<Event>>;
    async fn find_event_ids(&self) -> Result<Vec<EventId>>;
    async fn find_event_ids_by_event_id_after(&self, event_id: EventId) -> Result<Vec<EventId>>;
}

pub trait HasEventStore {
    type EventStore: EventStore + Send + Sync;

    fn event_store(&self) -> &Self::EventStore;
}
