use async_trait::async_trait;
use domain::aggregate::{user::UserRequestId, user_request::UserRequest};

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("unknown {0}")]
    Unknown(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[async_trait]
pub trait UserRequestRepository {
    async fn find(&self, id: UserRequestId) -> Result<Option<UserRequest>>;
    async fn store(&self, before: Option<UserRequest>, after: UserRequest) -> Result<()>;
}

pub trait HasUserRequestRepository {
    type UserRequestRepository: UserRequestRepository + Send + Sync;

    fn user_request_repository(&self) -> &Self::UserRequestRepository;
}
