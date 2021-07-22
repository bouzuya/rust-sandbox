use hatena_blog::EntryId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HatenaBlogEntryId(EntryId);

impl std::fmt::Display for HatenaBlogEntryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for HatenaBlogEntryId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(EntryId::from_str(s)?))
    }
}

impl From<EntryId> for HatenaBlogEntryId {
    fn from(entry_id: EntryId) -> Self {
        Self(entry_id)
    }
}

impl From<&HatenaBlogEntryId> for EntryId {
    fn from(hatena_blog_entry_id: &HatenaBlogEntryId) -> Self {
        hatena_blog_entry_id.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use hatena_blog::EntryId;

    use super::*;

    #[test]
    fn entry_id_conversion_test() {
        let entry_id = EntryId::from_str("ABC").unwrap();
        assert_eq!(
            EntryId::from(&HatenaBlogEntryId::from(entry_id.clone())),
            entry_id
        );
    }

    #[test]
    fn string_conversion_test() {
        let s = "ABC";
        assert_eq!(HatenaBlogEntryId::from_str(s).unwrap().to_string(), s);
    }
}
