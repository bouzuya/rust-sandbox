pub use event_store_core::event_store::{Error, EventStore, Result};

pub trait HasEventStore {
    type EventStore: EventStore + Send + Sync;

    fn event_store(&self) -> &Self::EventStore;
}
