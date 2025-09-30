use crate::in_memory_impls::InMemoryEventStore;
use crate::in_memory_impls::InMemoryReadModelStore;

#[derive(Debug, thiserror::Error)]
enum InMemoryUserWriterError {
    #[error("event store lock poisoned")]
    EventStoreLockPoisoned,
    #[error("query user from events")]
    QueryUserFromEvents(#[source] crate::query_models::QueryUserError),
    #[error("read model store lock poisoned")]
    ReadModelStoreLockPoisoned,
}

impl From<InMemoryUserWriterError> for crate::writers::UserWriterError {
    fn from(e: InMemoryUserWriterError) -> Self {
        crate::writers::UserWriterError(Box::new(e))
    }
}

pub struct InMemoryUserWriter {
    event_store: InMemoryEventStore,
    read_model_store: InMemoryReadModelStore,
}

impl InMemoryUserWriter {
    pub fn new(event_store: InMemoryEventStore, read_model_store: InMemoryReadModelStore) -> Self {
        Self {
            event_store,
            read_model_store,
        }
    }
}

#[async_trait::async_trait]
impl crate::writers::UserWriter for InMemoryUserWriter {
    async fn update(
        &self,
        id: &crate::value_objects::UserId,
    ) -> Result<(), crate::writers::UserWriterError> {
        let event_store = self
            .event_store
            .0
            .lock()
            .map_err(|_| InMemoryUserWriterError::EventStoreLockPoisoned)?;
        let mut read_model_store = self
            .read_model_store
            .0
            .lock()
            .map_err(|_| InMemoryUserWriterError::ReadModelStoreLockPoisoned)?;

        let events = match event_store.get(&String::from(id)) {
            None => return Ok(()),
            Some(events) => events,
        };

        let query_user = crate::query_models::QueryUser::from_events(events.iter().cloned())
            .map_err(InMemoryUserWriterError::QueryUserFromEvents)?;

        match read_model_store
            .iter_mut()
            .find(|it| it.id == query_user.id)
        {
            None => read_model_store.push(query_user),
            Some(existing) => *existing = query_user,
        }

        Ok(())
    }
}
