use async_trait::async_trait;

use crate::user::User;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("unknown {0}")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[async_trait]
pub trait UserStore {
    async fn find_by_twitter_user_id(&self, twitter_user_id: &String) -> Result<Option<User>>;
    async fn store(&self, before: Option<User>, after: User) -> Result<()>;
}

pub trait HasUserStore {
    type UserStore: UserStore + Send + Sync;

    fn user_store(&self) -> &Self::UserStore;
}
