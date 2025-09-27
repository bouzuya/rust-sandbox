#[derive(Debug, thiserror::Error)]
#[error("user name")]
pub struct UserNameError(#[source] Box<dyn std::error::Error + Send + Sync>);

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UserName(String);

impl UserName {
    #[cfg(test)]
    pub fn new_for_testing() -> Self {
        let mut rng = rand::rng();
        let s = rand::distr::SampleString::sample_string(&rand::distr::Alphanumeric, &mut rng, 10);
        Self(s)
    }
}

impl std::convert::TryFrom<String> for UserName {
    type Error = UserNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // TODO: check max length, forbidden characters, etc.
        if value.is_empty() {
            Err(UserNameError("User name cannot be empty".into()))
        } else {
            Ok(UserName(value))
        }
    }
}

impl std::convert::From<UserName> for String {
    fn from(value: UserName) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_try_from_string_for_user_name() -> anyhow::Result<()> {
        assert_eq!(
            UserName::try_from("Alice".to_owned())?,
            UserName("Alice".to_owned())
        );
        assert!(UserName::try_from("".to_owned()).is_err());
        Ok(())
    }

    #[test]
    fn test_impl_from_user_name_for_string() -> anyhow::Result<()> {
        let s = "Alice".to_owned();
        let user_name = UserName::try_from(s.clone())?;
        assert_eq!(String::from(user_name), s);
        Ok(())
    }

    #[test]
    fn test_new_for_testing() {
        let len = 100;
        let mut set = std::collections::BTreeSet::new();
        for _ in 0..len {
            let user_name = UserName::new_for_testing();
            set.insert(user_name);
        }
        assert_eq!(set.len(), len);
    }
}
