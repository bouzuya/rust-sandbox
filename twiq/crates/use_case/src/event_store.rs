pub use event_store_core::event_store::{Error, EventStore};

pub trait HasEventStore {
    type EventStore: EventStore + Send + Sync;

    fn event_store(&self) -> &Self::EventStore;
}
