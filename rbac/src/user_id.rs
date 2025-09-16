#[derive(Debug, thiserror::Error)]
pub enum UserIdError {
    #[error("invalid format: {0}")]
    InvalidFormat(String),
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UserId(String);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for UserId {
    type Err = UserIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(UserIdError::InvalidFormat(s.to_owned()))
        } else {
            Ok(UserId(s.to_owned()))
        }
    }
}
