use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use entity::{User, UserId};

use crate::port::{UserRepository, UserRepositoryError};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct InMemoryUserRepository {
    rc: Rc<RefCell<BTreeMap<UserId, User>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl UserRepository for InMemoryUserRepository {
    fn find_by_id(&self, user_id: UserId) -> Result<Option<User>, UserRepositoryError> {
        let storage = self.rc.borrow();
        Ok(storage.get(&user_id).cloned())
    }

    fn save(&self, user: User) -> Result<(), UserRepositoryError> {
        let mut storage = self.rc.borrow_mut();
        storage.insert(user.id(), user);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let repository = InMemoryUserRepository::default();
        let user = User::new();
        repository.save(user.clone())?;
        assert_eq!(repository.find_by_id(user.id())?, Some(user));
        Ok(())
    }
}
