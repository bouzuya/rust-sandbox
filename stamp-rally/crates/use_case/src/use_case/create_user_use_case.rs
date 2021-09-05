use entity::{User, UserId};
use thiserror::Error;

use crate::port::{HasUserRepository, UserRepository};

#[derive(Debug, Eq, Error, PartialEq)]
#[error("create stamp rally error")]
pub struct CreateUserError;

pub trait CreateUserUseCase: HasUserRepository {
    fn handle(&self) -> Result<UserId, CreateUserError> {
        let user_repository = self.user_repository();
        let user = User::new();
        let user_id = user.id();
        user_repository
            .save(user)
            .map(|_| user_id)
            .map_err(|_| CreateUserError)
    }
}

impl<T: HasUserRepository> CreateUserUseCase for T {}

pub trait HasCreateUserUseCase {
    type CreateUserUseCase: CreateUserUseCase;

    fn create_user_use_case(&self) -> &Self::CreateUserUseCase;
}

#[cfg(test)]
mod tests {
    use crate::InMemoryUserRepository;

    use super::*;

    struct U {
        user_repository: InMemoryUserRepository,
    }

    impl U {
        fn new_create_user_use_case() -> impl CreateUserUseCase {
            Self {
                user_repository: InMemoryUserRepository::new(),
            }
        }
    }

    impl HasUserRepository for U {
        type UserRepository = InMemoryUserRepository;

        fn user_repository(&self) -> &Self::UserRepository {
            &self.user_repository
        }
    }

    #[test]
    fn test() -> anyhow::Result<()> {
        let use_case = U::new_create_user_use_case();

        let user_id = CreateUserUseCase::handle(&use_case)?;

        assert!(use_case.user_repository().find_by_id(user_id)?.is_some());
        Ok(())
    }
}
