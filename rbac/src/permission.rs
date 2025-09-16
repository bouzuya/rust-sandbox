#[derive(Debug, thiserror::Error)]
pub enum PermissionError {
    #[error("unknown permission: {0}")]
    UnknownPermission(String),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Permission {
    A,
    B,
    C,
    // ...
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Permission::A => "A",
            Permission::B => "B",
            Permission::C => "C",
        }
        .fmt(f)
    }
}

impl std::str::FromStr for Permission {
    type Err = PermissionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Permission::A),
            "B" => Ok(Permission::B),
            "C" => Ok(Permission::C),
            _ => Err(PermissionError::UnknownPermission(s.to_owned())),
        }
    }
}
