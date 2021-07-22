use std::str::FromStr;

use hatena_blog::{Entry, EntryId};

use crate::{data::DateTime, hatena_blog::HatenaBlogEntryId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HatenaBlogEntry {
    pub author_name: String,
    pub categories: Vec<String>,
    pub content: String,
    pub draft: bool,
    pub edited: String,
    pub id: HatenaBlogEntryId,
    pub published: String,
    pub title: String,
    pub updated: DateTime,
}

impl From<Entry> for HatenaBlogEntry {
    fn from(entry: Entry) -> Self {
        Self {
            author_name: entry.author_name,
            categories: entry.categories,
            content: entry.content,
            draft: entry.draft,
            edited: entry.edited,
            id: HatenaBlogEntryId::from(entry.id),
            published: entry.published,
            title: entry.title,
            updated: DateTime::from_str(entry.updated.as_str()).expect("invalid entry.updated"),
        }
    }
}

impl From<HatenaBlogEntry> for Entry {
    fn from(hatena_blog_entry: HatenaBlogEntry) -> Self {
        Self {
            author_name: hatena_blog_entry.author_name,
            categories: hatena_blog_entry.categories,
            content: hatena_blog_entry.content,
            draft: hatena_blog_entry.draft,
            edited: hatena_blog_entry.edited,
            id: EntryId::from(&hatena_blog_entry.id),
            published: hatena_blog_entry.published,
            title: hatena_blog_entry.title,
            updated: hatena_blog_entry.updated.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn entry_conversion_test() {
        // TODO:
    }
}
