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
        let mut aggregate_ids = self.aggregate_ids.lock().await;
        let aggregate_id = after.id();
        let event_stream = EventStream::from(after);
        let event_stream_id = event_stream.id();

        // check the uniqueness of the aggregate_id
        match (&before, aggregate_ids.get(&aggregate_id)) {
            (None, None) => Ok(()),
            (None, Some(_)) => return Err(Error::Unknown("already exists".to_owned())),
            (Some(_), None) => return Err(Error::Unknown("not found".to_owned())),
            (Some(_), Some(_)) => Ok(()),
        }?;

        self.event_store
            .store(
                before.map(|aggregate| EventStream::from(aggregate).seq()),
                event_stream,
            )
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?;
        aggregate_ids.insert(aggregate_id, event_stream_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use domain::aggregate::user::{TwitterUserId, UserId};

    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let user_id = UserId::generate();
        let id = UserRequestId::generate();
        let twitter_user_id = TwitterUserId::from_str("125962981")?;
        let mut user_request = UserRequest::create(id, twitter_user_id, user_id)?;
        let repository = InMemoryUserRequestRepository::default();
        assert!(repository.find(user_request.id()).await?.is_none());
        repository.store(None, user_request.clone()).await?;
        assert_eq!(
            repository.find(user_request.id()).await?,
            Some(user_request.clone())
        );
        let before = user_request.clone();
        user_request.start()?;
        repository
            .store(Some(before.clone()), user_request.clone())
            .await?;
        assert_eq!(
            repository.find(user_request.id()).await?,
            Some(user_request.clone())
        );

        // store twice
        assert!(repository
            .store(Some(before.clone()), user_request.clone())
            .await
            .is_err());

        // duplicate id
        let user_id = UserId::generate();
        let twitter_user_id = TwitterUserId::from_str("125962981")?;
        let user_request2 = UserRequest::create(id, twitter_user_id, user_id)?;
        assert!(repository.store(None, user_request2.clone()).await.is_err());

        Ok(())
    }
}
