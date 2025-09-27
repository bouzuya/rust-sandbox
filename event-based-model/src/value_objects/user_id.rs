#[derive(Debug, thiserror::Error)]
#[error("user id")]
pub struct UserIdError(#[source] Box<dyn std::error::Error + Send + Sync>);

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UserId(String);

impl UserId {
    pub fn new() -> Self {
        UserId(uuid::Uuid::new_v4().to_string())
    }

    #[cfg(test)]
    pub fn new_for_testing() -> Self {
        Self::new()
    }
}

impl std::convert::TryFrom<String> for UserId {
    type Error = UserIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 36 {
            return Err(UserIdError("invalid length".into()));
        }
        if !value
            .chars()
            .all(|c| c == '-' || ('0'..='9').contains(&c) || ('a'..='f').contains(&c))
        {
            return Err(UserIdError("invalid characters".into()));
        }
        let v = <uuid::Uuid as std::str::FromStr>::from_str(&value)
            .map_err(|_| UserIdError("invalid format".into()))?;
        if v.get_version_num() != 4 {
            return Err(UserIdError("invalid value".into()));
        }
        Ok(UserId(value))
    }
}

impl std::convert::From<UserId> for String {
    fn from(value: UserId) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_try_from_string_for_user_id() -> anyhow::Result<()> {
        let s = uuid::Uuid::new_v4().to_string();
        assert_eq!(UserId::try_from(s.clone())?, UserId(s));
        assert!(UserId::try_from("0".repeat(36 + 1)).is_err());
        assert!(UserId::try_from("A".repeat(36 + 1)).is_err());
        assert!(UserId::try_from("".to_owned()).is_err());
        Ok(())
    }

    #[test]
    fn test_impl_from_user_id_for_string() -> anyhow::Result<()> {
        let user_id = UserId::new();
        let s = String::from(user_id.clone());
        let user_id2 = UserId::try_from(s.clone())?;
        assert_eq!(user_id, user_id2);
        Ok(())
    }

    #[test]
    fn test_new() -> anyhow::Result<()> {
        let user_id = UserId::new();
        assert_eq!(String::from(user_id.clone()).len(), 36);
        assert_eq!(
            <uuid::Uuid as std::str::FromStr>::from_str(&String::from(user_id))?.get_version_num(),
            4
        );
        Ok(())
    }

    #[test]
    fn test_new_for_testing() {
        let len = 100;
        let mut set = std::collections::BTreeSet::new();
        for _ in 0..len {
            let user_id = UserId::new_for_testing();
            set.insert(user_id);
        }
        assert_eq!(set.len(), len);
    }
}
