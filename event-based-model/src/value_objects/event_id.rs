#[derive(Debug, thiserror::Error)]
#[error("event id")]
pub struct EventIdError(#[source] Box<dyn std::error::Error + Send + Sync>);

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EventId(String);

impl EventId {
    pub fn new() -> Self {
        EventId(uuid::Uuid::new_v4().to_string())
    }

    #[cfg(test)]
    pub fn new_for_testing() -> Self {
        Self::new()
    }
}

impl std::convert::TryFrom<String> for EventId {
    type Error = EventIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 36 {
            return Err(EventIdError("invalid length".into()));
        }
        if !value
            .chars()
            .all(|c| c == '-' || ('0'..='9').contains(&c) || ('a'..='f').contains(&c))
        {
            return Err(EventIdError("invalid characters".into()));
        }
        let v = <uuid::Uuid as std::str::FromStr>::from_str(&value)
            .map_err(|_| EventIdError("invalid format".into()))?;
        if v.get_version_num() != 4 {
            return Err(EventIdError("invalid value".into()));
        }
        Ok(EventId(value))
    }
}

impl std::convert::From<&EventId> for String {
    fn from(value: &EventId) -> Self {
        value.0.clone()
    }
}

impl std::convert::From<EventId> for String {
    fn from(value: EventId) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_try_from_string_for_user_id() -> anyhow::Result<()> {
        let s = uuid::Uuid::new_v4().to_string();
        assert_eq!(EventId::try_from(s.clone())?, EventId(s));
        assert!(EventId::try_from("0".repeat(36 + 1)).is_err());
        assert!(EventId::try_from("A".repeat(36 + 1)).is_err());
        assert!(EventId::try_from("".to_owned()).is_err());
        Ok(())
    }

    #[test]
    fn test_impl_from_user_id_for_string() -> anyhow::Result<()> {
        let user_id = EventId::new();
        let s = String::from(user_id.clone());
        let user_id2 = EventId::try_from(s.clone())?;
        assert_eq!(user_id, user_id2);
        Ok(())
    }

    #[test]
    fn test_new() -> anyhow::Result<()> {
        let user_id = EventId::new();
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
            let user_id = EventId::new_for_testing();
            set.insert(user_id);
        }
        assert_eq!(set.len(), len);
    }
}
