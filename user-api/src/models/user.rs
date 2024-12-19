use crate::models::user_id::UserId;
use crate::models::user_secret::UserSecret;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    pub(crate) id: UserId,
    pub(crate) name: String,
    pub(crate) secret: UserSecret,
}

impl User {
    pub fn new(name: String) -> anyhow::Result<(Self, String)> {
        if name.is_empty() {
            anyhow::bail!("name is empty");
        }
        let (secret, raw) = UserSecret::generate()?;
        Ok((
            Self {
                id: UserId::generate(),
                name,
                secret,
            },
            raw,
        ))
    }

    pub fn update(&mut self, name: String) -> anyhow::Result<()> {
        if name.is_empty() {
            anyhow::bail!("name is empty");
        }
        self.name = name;
        Ok(())
    }
}
