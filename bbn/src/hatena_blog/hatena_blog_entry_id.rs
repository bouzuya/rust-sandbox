use hatena_blog::EntryId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HatenaBlogEntryId(EntryId);

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
}
