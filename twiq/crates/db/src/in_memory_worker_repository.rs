use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use event_store_core::EventId;
use tokio::sync::Mutex;
use use_case::worker_repository::{Result, WorkerName, WorkerRepository};

#[derive(Debug, Default)]
pub struct InMemoryWorkerRepository {
    _data: Arc<Mutex<HashMap<WorkerName, EventId>>>,
}

#[async_trait]
impl WorkerRepository for InMemoryWorkerRepository {
    async fn find_last_event_id(&self, _worker_name: WorkerName) -> Result<Option<EventId>> {
        todo!()
    }

    async fn store_last_event_id(
        &self,
        _worker_name: WorkerName,
        _before: Option<EventId>,
        _after: EventId,
    ) -> Result<()> {
        todo!()
    }
}
