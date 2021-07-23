use crate::data::DateTime;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EntryMeta {
    pub hatena_blog_ignore: Option<bool>,
    pub minutes: u64,
    pub pubdate: DateTime,
    pub tags: Vec<String>,
    pub title: String,
}
