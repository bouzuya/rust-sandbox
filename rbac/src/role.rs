#[derive(Debug, thiserror::Error)]
pub enum RoleError {
    #[error("unknown role: {0}")]
    UnknownRole(String),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Role {
    Admin,
    User,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => "admin",
            Role::User => "user",
        }
        .fmt(f)
    }
}

impl std::str::FromStr for Role {
    type Err = RoleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(Role::Admin),
            "user" => Ok(Role::User),
            _ => Err(RoleError::UnknownRole(s.to_owned())),
        }
    }
}
