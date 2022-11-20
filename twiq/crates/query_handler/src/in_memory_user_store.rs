use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use tracing::instrument;

use crate::{
    user::User,
    user_store::{Result, UserStore},
};

type UserId = String;
type TwitterUserId = String;
type TwitterUserName = String;

#[derive(Clone, Debug, Default)]
pub struct InMemoryUserStore {
    users: Arc<Mutex<HashMap<UserId, User>>>,
    twitter_user_ids: Arc<Mutex<HashMap<TwitterUserId, UserId>>>,
    twitter_user_names: Arc<Mutex<HashMap<TwitterUserName, UserId>>>,
}

#[async_trait]
impl UserStore for InMemoryUserStore {
    #[instrument]
    async fn find_by_twitter_user_id(&self, twitter_user_id: &str) -> Result<Option<User>> {
        let users = self.users.lock().unwrap();
        let twitter_user_ids = self.twitter_user_ids.lock().unwrap();
        Ok(twitter_user_ids
            .get(twitter_user_id)
            .and_then(|user_id| users.get(user_id).cloned()))
    }

    #[instrument]
    async fn store(&self, before: Option<User>, after: User) -> Result<()> {
        let mut users = self.users.lock().unwrap();
        let mut twitter_user_ids = self.twitter_user_ids.lock().unwrap();
        let mut twitter_user_names = self.twitter_user_names.lock().unwrap();

        match before {
            None => {}
            Some(before) => {
                users.remove(&before.user_id);
                twitter_user_ids.remove(&before.twitter_user_id);
                twitter_user_names.remove(&before.twitter_user_name);
            }
        }

        let user = users.get_mut(&after.user_id);
        let twitter_user_id = twitter_user_ids.get_mut(&after.twitter_user_id);
        let twitter_user_name = twitter_user_names.get_mut(&after.twitter_user_name);
        match (user, twitter_user_id, twitter_user_name) {
            (None, None, None) => {
                users.insert(after.user_id.clone(), after.clone());
                twitter_user_ids.insert(after.twitter_user_id.clone(), after.user_id.clone());
                twitter_user_names.insert(after.twitter_user_name.clone(), after.user_id.clone());
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let user_store = InMemoryUserStore::default();
        let user1 = User {
            user_id: "user_id1".to_owned(),
            twitter_user_id: "twitter_user_id1".to_owned(),
            twitter_user_name: "twitter_user_name1".to_owned(),
        };
        user_store.store(None, user1.clone()).await?;
        assert!(user_store
            .find_by_twitter_user_id("twitter_user_id2")
            .await?
            .is_none());
        assert_eq!(
            user_store
                .find_by_twitter_user_id("twitter_user_id1")
                .await?,
            Some(user1.clone())
        );
        let user2 = User {
            user_id: "user_id1".to_owned(),
            twitter_user_id: "twitter_user_id2".to_owned(),
            twitter_user_name: "twitter_user_name2".to_owned(),
        };
        user_store.store(Some(user1), user2.clone()).await?;
        assert_eq!(
            user_store
                .find_by_twitter_user_id("twitter_user_id2")
                .await?,
            Some(user2)
        );
        Ok(())
    }
}
