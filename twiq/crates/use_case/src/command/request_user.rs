use async_trait::async_trait;
use domain::aggregate::user::{At, TwitterUserId, User};

use crate::user_repository::{HasUserRepository, UserRepository};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("user_aggregate {0}")]
    UserAggregate(#[from] domain::aggregate::user::Error),
    #[error("user_repository {0}")]
    UserRepository(#[from] crate::user_repository::Error),
    #[error("unknown {0}")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Command {
    pub twitter_user_id: TwitterUserId,
}

pub trait Context: HasUserRepository {}

impl<T: HasUserRepository> Context for T {}

#[async_trait]
pub trait Has: Context + Sized {
    async fn request_user(&self, command: Command) -> Result<()> {
        handler(self, command).await
    }
}

pub async fn handler<C: Context>(context: &C, command: Command) -> Result<()> {
    let user_repository = context.user_repository();
    let twitter_user_id = command.twitter_user_id;
    let found = user_repository
        .find_by_twitter_user_id(&twitter_user_id)
        .await?;
    let mut updated = match found.clone() {
        None => User::create(twitter_user_id)?,
        Some(user) => user.clone(),
    };
    // TODO: &mut self -> &self
    updated.request(At::now())?;
    user_repository.store(found, updated).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anyhow::Context;

    use crate::in_memory_user_repository::InMemoryUserRepository;

    use super::*;

    #[derive(Debug, Default)]
    struct MockApp {
        user_repository: InMemoryUserRepository,
    }

    impl HasUserRepository for MockApp {
        type UserRepository = InMemoryUserRepository;

        fn user_repository(&self) -> &Self::UserRepository {
            &self.user_repository
        }
    }

    impl Has for MockApp {}

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let twitter_user_id = TwitterUserId::from_str("125962981")?;
        let context = MockApp::default();
        context
            .request_user(Command {
                twitter_user_id: twitter_user_id.clone(),
            })
            .await?;
        let user = context
            .user_repository
            .find_by_twitter_user_id(&twitter_user_id)
            .await?
            .context("user not found")?;
        assert_eq!(user.twitter_user_id(), &twitter_user_id);
        Ok(())
    }
}
