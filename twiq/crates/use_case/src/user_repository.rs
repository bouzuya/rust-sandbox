use async_trait::async_trait;
use domain::aggregate::user::{TwitterUserId, User, UserId};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("unknown {0}")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[async_trait]
pub trait UserRepository {
    async fn find(&self, id: UserId) -> Result<Option<User>>;
    async fn find_by_twitter_user_id(
        &self,
        twitter_user_id: &TwitterUserId,
    ) -> Result<Option<User>>;
    async fn store(&self, before: Option<User>, after: User) -> Result<()>;
}

pub trait HasUserRepository {
    type UserRepository: UserRepository + Send + Sync;

    fn user_repository(&self) -> &Self::UserRepository;
}
