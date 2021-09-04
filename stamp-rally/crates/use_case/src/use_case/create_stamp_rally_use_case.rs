use entity::{StampRally, StampRallyId};
use thiserror::Error;

use crate::port::{HasStampRallyRepository, StampRallyRepository};

#[derive(Debug, Eq, Error, PartialEq)]
#[error("create stamp rally error")]
pub struct CreateStampRallyError;

pub trait CreateStampRallyUseCase: HasStampRallyRepository {
    fn handle(&self) -> Result<StampRallyId, CreateStampRallyError> {
        let stamp_rally_repository = self.stamp_rally_repository();
        let stamp_rally = StampRally::new();
        stamp_rally_repository
            .save(stamp_rally.clone())
            .map(|_| stamp_rally.id())
            .map_err(|_| CreateStampRallyError)
    }
}

impl<T: HasStampRallyRepository> CreateStampRallyUseCase for T {}

pub trait HasCreateStampRallyUseCase {
    type CreateStampRallyUseCase: CreateStampRallyUseCase;

    fn create_stamp_rally_use_case(&self) -> &Self::CreateStampRallyUseCase;
}

#[cfg(test)]
mod tests {
    use crate::InMemoryStampRallyRepository;

    use super::*;

    struct U {
        stamp_rally_repository: InMemoryStampRallyRepository,
    }

    impl U {
        fn new_create_stamp_rally_use_case() -> impl CreateStampRallyUseCase {
            Self {
                stamp_rally_repository: InMemoryStampRallyRepository::new(),
            }
        }
    }

    impl HasStampRallyRepository for U {
        type StampRallyRepository = InMemoryStampRallyRepository;

        fn stamp_rally_repository(&self) -> &Self::StampRallyRepository {
            &self.stamp_rally_repository
        }
    }

    #[test]
    fn test() -> anyhow::Result<()> {
        let use_case = U::new_create_stamp_rally_use_case();

        let stamp_rally_id = CreateStampRallyUseCase::handle(&use_case)?;

        assert!(use_case
            .stamp_rally_repository()
            .find_by_id(stamp_rally_id)?
            .is_some());
        Ok(())
    }
}
