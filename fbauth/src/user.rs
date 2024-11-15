use crate::{user_id::UserId, user_secret::UserSecret};

#[derive(Clone, Eq, PartialEq)]
pub struct User {
    pub(crate) id: UserId,
    pub(crate) secret: UserSecret,
}

impl User {
    pub fn new() -> Self {
        Self {
            id: UserId::generate(),
            secret: UserSecret::generate(),
        }
    }
}
