use crate::timestamp::Timestamp;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EntryMeta {
    pub minutes: u64,
    pub pubdate: Timestamp,
    pub tags: Vec<String>,
    pub title: String,
}
