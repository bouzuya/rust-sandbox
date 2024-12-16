use crate::models::user_id::UserId;
use crate::models::user_secret::UserSecret;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    pub(crate) id: UserId,
    pub(crate) secret: UserSecret,
}

impl User {
    pub fn new() -> anyhow::Result<(Self, String)> {
        let (secret, raw) = UserSecret::generate()?;
        Ok((
            Self {
                id: UserId::generate(),
                secret,
            },
            raw,
        ))
    }
}
