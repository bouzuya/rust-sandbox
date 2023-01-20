use crate::data::DateTime;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryMeta {
    pub hatena_blog_entry_id: Option<String>,
    pub hatena_blog_entry_url: Option<String>,
    pub hatena_blog_ignore: Option<bool>,
    pub minutes: u64,
    pub pubdate: DateTime,
    pub tags: Vec<String>,
    pub title: String,
}

impl EntryMeta {
    pub fn new(minutes: u64, pubdate: DateTime, tags: Vec<String>, title: String) -> Self {
        Self {
            hatena_blog_entry_id: None,
            hatena_blog_entry_url: None,
            hatena_blog_ignore: None,
            minutes,
            pubdate,
            tags,
            title,
        }
    }
}
