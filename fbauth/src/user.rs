use crate::user_id::UserId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    pub(crate) id: UserId,
}

impl User {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            id: UserId::generate(),
        })
    }
}
