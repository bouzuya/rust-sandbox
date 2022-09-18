use std::{
    borrow::BorrowMut,
    collections::HashMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use domain::aggregate::user::{TwitterUserId, User, UserId};
use use_case::user_repository::{Error, Result, UserRepository};

pub struct InMemoryUserRepository {
    data: Arc<Mutex<HashMap<UserId, User>>>,
    index: Arc<Mutex<HashMap<TwitterUserId, UserId>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
            index: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn find(&self, id: UserId) -> Result<Option<User>> {
        let data = self
            .data
            .as_ref()
            .lock()
            .map_err(|e| Error::Unknown(e.to_string()))?;
        Ok(data.get(&id).cloned())
    }

    async fn find_by_twitter_user_id(
        &self,
        twitter_user_id: &TwitterUserId,
    ) -> Result<Option<User>> {
        let user_id = {
            let index = self
                .index
                .as_ref()
                .lock()
                .map_err(|e| Error::Unknown(e.to_string()))?;
            match index.get(twitter_user_id).copied() {
                Some(user_id) => user_id,
                None => return Ok(None),
            }
        };
        self.find(user_id).await
    }

    async fn store(&self, before: Option<User>, after: User) -> Result<()> {
        let mut data = self
            .data
            .lock()
            .map_err(|e| Error::Unknown(e.to_string()))?;
        let data = data.borrow_mut();
        if let Some(before) = before {
            if data.get(&before.id()) != Some(&before) {
                return Err(Error::Unknown("conflict".to_owned()));
            }
        }
        data.insert(after.id(), after);
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
        let repository = InMemoryUserRepository::new();
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
