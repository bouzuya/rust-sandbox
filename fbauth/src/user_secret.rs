// FIXME: use safe secret
#[derive(Clone, Eq, PartialEq)]
pub struct UserSecret(uuid::Uuid);

impl UserSecret {
    pub fn generate() -> Self {
        UserSecret(uuid::Uuid::new_v4())
    }
}

impl std::fmt::Display for UserSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for UserSecret {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(uuid::Uuid::from_str(s)?))
    }
}
