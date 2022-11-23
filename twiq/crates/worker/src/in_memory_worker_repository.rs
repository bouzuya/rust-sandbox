use std::{collections::HashMap, sync::Arc};

use crate::worker_repository::{Error, Result, WorkerName, WorkerRepository};
use async_trait::async_trait;
use event_store_core::{event_store::EventStore, Event, EventId};
use tokio::sync::Mutex;
use tracing::{debug, instrument};
use use_case::in_memory_event_store::InMemoryEventStore;

#[derive(Debug)]
pub struct InMemoryWorkerRepository {
    event_store: InMemoryEventStore,
    data: Arc<Mutex<HashMap<WorkerName, EventId>>>,
}

impl InMemoryWorkerRepository {
    pub fn new(empty_event_store: InMemoryEventStore) -> Self {
        Self {
            event_store: empty_event_store,
            data: Default::default(),
        }
    }
}

#[async_trait]
impl WorkerRepository for InMemoryWorkerRepository {
    async fn find_event_ids(&self, event_id: Option<EventId>) -> Result<Vec<EventId>> {
        self.event_store
            .find_event_ids(event_id)
            .await
            .map_err(|e| Error::Unknown(e.to_string()))
    }

    async fn find_event(&self, event_id: EventId) -> Result<Option<Event>> {
        self.event_store
            .find_event(event_id)
            .await
            .map_err(|e| Error::Unknown(e.to_string()))
    }

    async fn find_last_event_id(&self, worker_name: WorkerName) -> Result<Option<EventId>> {
        let data = self.data.lock().await;
        Ok(data.get(&worker_name).cloned())
    }

    #[instrument(skip_all)]
    async fn store_last_event_id(
        &self,
        worker_name: WorkerName,
        before: Option<EventId>,
        after: EventId,
    ) -> Result<()> {
        let mut data = self.data.lock().await;
        let current = data.get_mut(&worker_name);
        debug!(
            "{:?} {:?} -> {:?} ({:?})",
            worker_name, before, after, current
        );
        match (before, current) {
            (None, None) => {
                data.insert(worker_name, after);
                Ok(())
            }
            (None, Some(_)) | (Some(_), None) => Err(Error::Unknown("conflict".to_owned())),
            (Some(b), Some(c)) => {
                if b == *c {
                    *c = after;
                    Ok(())
                } else {
                    Err(Error::Unknown("conflict".to_owned()))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let event_store = InMemoryEventStore::default();
        let repository = InMemoryWorkerRepository::new(event_store);
        let worker_name1 = WorkerName::CreateUserRequest;
        let worker_name2 = WorkerName::UpdateUser;

        let found = repository.find_last_event_id(worker_name1).await?;
        assert!(found.is_none());

        let event_id1 = EventId::generate();
        repository
            .store_last_event_id(worker_name1, found, event_id1)
            .await?;
        let found = repository.find_last_event_id(worker_name1).await?;
        assert_eq!(found, Some(event_id1));
        assert!(repository.find_last_event_id(worker_name2).await?.is_none());

        let event_id2 = EventId::generate();
        assert!(repository
            .store_last_event_id(worker_name1, None, event_id2)
            .await
            .is_err());
        assert_eq!(
            repository.find_last_event_id(worker_name1).await?,
            Some(event_id1)
        );
        repository
            .store_last_event_id(worker_name1, found, event_id2)
            .await?;
        assert_eq!(
            repository.find_last_event_id(worker_name1).await?,
            Some(event_id2)
        );
        assert!(repository.find_last_event_id(worker_name2).await?.is_none());
        Ok(())
    }
}
