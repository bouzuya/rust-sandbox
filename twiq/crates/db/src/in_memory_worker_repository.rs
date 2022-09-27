use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use event_store_core::EventId;
use tokio::sync::Mutex;
use use_case::worker_repository::{Error, Result, WorkerName, WorkerRepository};

#[derive(Debug, Default)]
pub struct InMemoryWorkerRepository {
    data: Arc<Mutex<HashMap<WorkerName, EventId>>>,
}

#[async_trait]
impl WorkerRepository for InMemoryWorkerRepository {
    async fn find_last_event_id(&self, worker_name: WorkerName) -> Result<Option<EventId>> {
        let data = self.data.lock().await;
        Ok(data.get(&worker_name).cloned())
    }

    async fn store_last_event_id(
        &self,
        worker_name: WorkerName,
        before: Option<EventId>,
        after: EventId,
    ) -> Result<()> {
        let mut data = self.data.lock().await;
        let current = data.get_mut(&worker_name);
        match (before, current) {
            (None, None) => {
                data.insert(worker_name, after);
                Ok(())
            }
            (None, Some(_)) | (Some(_), None) => Err(Error::Unknown("conflict".to_owned())),
            (Some(b), Some(c)) => {
                if b == *c {
                    *c = b;
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
        let repository = InMemoryWorkerRepository::default();
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
        repository
            .store_last_event_id(worker_name1, found, event_id2)
            .await?;
        Ok(())
    }
}
