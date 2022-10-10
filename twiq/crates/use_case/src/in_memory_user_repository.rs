use std::{collections::HashMap, sync::Arc};

use crate::user_repository::{Error, Result, UserRepository};
use async_trait::async_trait;
use domain::aggregate::user::{TwitterUserId, User, UserId};
use event_store_core::{event_store::EventStore, EventStream, EventStreamId};
use tokio::sync::Mutex;

use crate::in_memory_event_store::InMemoryEventStore;

#[derive(Debug, Default)]
pub struct InMemoryUserRepository {
    event_store: InMemoryEventStore,
    user_ids: Arc<Mutex<HashMap<UserId, EventStreamId>>>,
    index: Arc<Mutex<HashMap<TwitterUserId, UserId>>>,
}

impl InMemoryUserRepository {
    pub fn new(empty_event_store: InMemoryEventStore) -> Self {
        Self {
            event_store: empty_event_store,
            user_ids: Default::default(),
            index: Default::default(),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn find(&self, id: UserId) -> Result<Option<User>> {
        let user_ids = self.user_ids.lock().await;
        let event_stream_id = match user_ids.get(&id) {
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
            Some(event_stream) => User::try_from(event_stream)
                .map(Some)
                .map_err(|e| Error::Unknown(e.to_string())),
        }
    }

    async fn find_by_twitter_user_id(
        &self,
        twitter_user_id: &TwitterUserId,
    ) -> Result<Option<User>> {
        let user_id = {
            let index = self.index.as_ref().lock().await;
            match index.get(twitter_user_id).copied() {
                Some(user_id) => user_id,
                None => return Ok(None),
            }
        };
        self.find(user_id).await
    }

    async fn store(&self, before: Option<User>, after: User) -> Result<()> {
        let mut user_ids = self.user_ids.lock().await;
        let mut index = self.index.lock().await;
        self.event_store
            .store(
                before.map(|user| EventStream::from(user).seq()),
                EventStream::from(after.clone()),
            )
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?;
        user_ids.insert(after.id(), after.event_stream().id());
        index.insert(after.twitter_user_id().clone(), after.id());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use domain::aggregate::user::{At, TwitterUserName};

    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let mut user = User::create(TwitterUserId::from_str("123")?)?;
        let repository = InMemoryUserRepository::default();
        assert!(repository.find(user.id()).await?.is_none());
        repository.store(None, user.clone()).await?;
        assert_eq!(repository.find(user.id()).await?, Some(user.clone()));
        let before = user.clone();
        user.update(TwitterUserName::from_str("twitter_user_name")?, At::now())?;
        repository.store(Some(before), user.clone()).await?;
        assert_eq!(repository.find(user.id()).await?, Some(user));
        Ok(())
    }
}
