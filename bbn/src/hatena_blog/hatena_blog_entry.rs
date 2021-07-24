use hatena_blog::{Entry, EntryId, FixedDateTime};

use crate::{data::DateTime, hatena_blog::HatenaBlogEntryId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HatenaBlogEntry {
    pub author_name: String,
    pub categories: Vec<String>,
    pub content: String,
    pub draft: bool,
    pub edit_url: String,
    pub edited: DateTime,
    pub id: HatenaBlogEntryId,
    pub published: DateTime,
    pub title: String,
    pub updated: DateTime,
    pub url: String,
}

impl From<Entry> for HatenaBlogEntry {
    fn from(entry: Entry) -> Self {
        Self {
            author_name: entry.author_name,
            categories: entry.categories,
            content: entry.content,
            draft: entry.draft,
            edit_url: entry.edit_url,
            edited: DateTime::from(entry.edited),
            id: HatenaBlogEntryId::from(entry.id),
            published: DateTime::from(entry.published),
            title: entry.title,
            updated: DateTime::from(entry.updated),
            url: entry.url,
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
            edit_url: hatena_blog_entry.edit_url,
            edited: FixedDateTime::from(hatena_blog_entry.edited),
            id: EntryId::from(&hatena_blog_entry.id),
            published: FixedDateTime::from(hatena_blog_entry.published),
            title: hatena_blog_entry.title,
            updated: FixedDateTime::from(hatena_blog_entry.updated),
            url: hatena_blog_entry.url,
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
