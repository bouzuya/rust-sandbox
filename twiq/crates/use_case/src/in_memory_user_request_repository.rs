use std::{collections::HashMap, sync::Arc};

use crate::user_request_repository::{Error, Result, UserRequestRepository};
use async_trait::async_trait;
use domain::aggregate::{user::UserRequestId, user_request::UserRequest};
use event_store_core::{event_store::EventStore, EventStream, EventStreamId};
use tokio::sync::Mutex;

use crate::in_memory_event_store::InMemoryEventStore;

#[derive(Debug, Default)]
pub struct InMemoryUserRequestRepository {
    event_store: InMemoryEventStore,
    aggregate_ids: Arc<Mutex<HashMap<UserRequestId, EventStreamId>>>,
}

impl InMemoryUserRequestRepository {
    pub fn new(empty_event_store: InMemoryEventStore) -> Self {
        Self {
            event_store: empty_event_store,
            aggregate_ids: Default::default(),
        }
    }
}

#[async_trait]
impl UserRequestRepository for InMemoryUserRequestRepository {
    async fn find(&self, id: UserRequestId) -> Result<Option<UserRequest>> {
        let aggregate_ids = self.aggregate_ids.lock().await;
        let event_stream_id = match aggregate_ids.get(&id) {
            None => return Ok(None),
            Some(event_stream_id) => *event_stream_id,
        };
        let event_stream = self
            .event_store
            .find_event_stream(event_stream_id)
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?;
        match event_stream {
            None => Ok(None),
            Some(event_stream) => UserRequest::try_from(event_stream)
                .map(Some)
                .map_err(|e| Error::Unknown(e.to_string())),
        }
    }

    async fn store(&self, before: Option<UserRequest>, after: UserRequest) -> Result<()> {
        let mut user_request_ids = self.aggregate_ids.lock().await;
        let aggregate_id = after.id();
        let event_stream = EventStream::from(after);
        let event_stream_id = event_stream.id();
        self.event_store
            .store(
                before.map(|user_request| EventStream::from(user_request).seq()),
                event_stream,
            )
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?;
        user_request_ids.insert(aggregate_id, event_stream_id);
        Ok(())
    }
}
