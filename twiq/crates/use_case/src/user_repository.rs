use async_trait::async_trait;
use domain::aggregate::user::{User, UserId};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("unknown {0}")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[async_trait]
pub trait UserRepository {
    async fn find(id: UserId) -> Result<Option<User>>;
    async fn store(before: Option<User>, after: User) -> Result<()>;
}
