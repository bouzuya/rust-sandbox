use crate::{google_account_id::GoogleAccountId, user_id::UserId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    pub(crate) google_account_id: GoogleAccountId,
    pub(crate) id: UserId,
}

impl User {
    pub fn new(google_account_id: GoogleAccountId) -> anyhow::Result<Self> {
        Ok(Self {
            google_account_id,
            id: UserId::generate(),
        })
    }
}
