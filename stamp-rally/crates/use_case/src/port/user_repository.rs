use entity::{User, UserId};
use thiserror::Error;

#[derive(Clone, Debug, Error, Eq, PartialEq)]
#[error("user repository error")]
pub struct UserRepositoryError;

pub trait UserRepository {
    fn find_by_id(&self, user_id: UserId) -> Result<Option<User>, UserRepositoryError>;
    fn save(&self, user: User) -> Result<(), UserRepositoryError>;
}

pub trait HasUserRepository {
    type UserRepository: UserRepository;

    fn user_repository(&self) -> &Self::UserRepository;
}
