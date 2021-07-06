use crate::{entry_id::EntryId, timestamp::Timestamp};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Entry {
    pub content: String,
    pub entry_id: EntryId,
    pub minutes: u64,
    pub pubdate: Timestamp,
    pub title: String,
    // TODO: tags: Vec<String>
}
