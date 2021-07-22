use crate::datetime::DateTime;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EntryMeta {
    pub minutes: u64,
    pub pubdate: DateTime,
    pub tags: Vec<String>,
    pub title: String,
}
