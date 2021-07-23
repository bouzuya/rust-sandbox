use crate::data::{EntryId, EntryMeta};

#[derive(Debug, Eq, PartialEq)]
pub struct Entry {
    id: EntryId,
    meta: EntryMeta,
    content: String,
}

impl Entry {
    pub fn new(id: EntryId, meta: EntryMeta, content: String) -> Self {
        Self { id, meta, content }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn id(&self) -> &EntryId {
        &self.id
    }

    pub fn meta(&self) -> &EntryMeta {
        &self.meta
    }

    pub fn update(self, content: String, meta: EntryMeta) -> Self {
        Self {
            id: self.id,
            meta,
            content,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use date_range::date::Date;

    use crate::data::DateTime;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let id = EntryId::new(Date::from_str("2021-02-03")?, None);
        let content = "content".to_string();
        let meta = EntryMeta {
            hatena_blog_ignore: None,
            minutes: 15,
            pubdate: DateTime::from_str("2021-02-03T16:17:18+09:00")?,
            tags: vec![],
            title: "title".to_string(),
        };
        let entry = Entry::new(id.clone(), meta.clone(), content.clone());
        assert_eq!(entry.content(), content.as_str());
        assert_eq!(entry.id(), &id);
        assert_eq!(entry.meta(), &meta);

        let content2 = "content2".to_string();
        let meta2 = EntryMeta {
            hatena_blog_ignore: None,
            minutes: 16,
            pubdate: DateTime::from_str("2021-02-04T16:17:18+09:00")?,
            tags: vec!["tag2".to_string()],
            title: "title2".to_string(),
        };
        let updated = entry.update(content2.clone(), meta2.clone());
        assert_eq!(updated.content(), content2.as_str());
        assert_eq!(updated.id(), &id);
        assert_eq!(updated.meta(), &meta2);
        Ok(())
    }
}
