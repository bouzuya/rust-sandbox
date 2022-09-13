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
    twitter_user_id: TwitterUserId,
}

pub async fn handler<C: HasUserRepository>(context: &C, command: Command) -> Result<()> {
    let user_repository = context.user_repository();
    let twitter_user_id = command.twitter_user_id;
    let found = user_repository
        .find_by_twitter_user_id(&twitter_user_id)
        .await?;
    let updated = match found.clone() {
        None => User::create(twitter_user_id)?,
        Some(mut user) => {
            // TODO: &mut self -> &self
            user.request(At::now())?;
            user
        }
    };
    user_repository.store(found, updated).await?;
    Ok(())
}

// TODO: test
