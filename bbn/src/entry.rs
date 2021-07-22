use crate::entry_meta::EntryMeta;

#[derive(Debug, Eq, PartialEq)]
pub struct Entry {
    content: String,
    meta: EntryMeta,
}

impl Entry {
    pub fn new(content: String, meta: EntryMeta) -> Self {
        Self { content, meta }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn meta(&self) -> &EntryMeta {
        &self.meta
    }

    pub fn update(self, content: String, meta: EntryMeta) -> Self {
        Self { content, meta }
    }
}

#[cfg(test)]
mod tests {
    use crate::timestamp::Timestamp;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let content = "content".to_string();
        let meta = EntryMeta {
            minutes: 15,
            pubdate: Timestamp::now()?,
            tags: vec![],
            title: "title".to_string(),
        };
        let entry = Entry::new(content.clone(), meta.clone());
        assert_eq!(entry.content(), content.as_str());
        assert_eq!(entry.meta(), &meta);

        let content2 = "content2".to_string();
        let meta2 = EntryMeta {
            minutes: 16,
            pubdate: Timestamp::now()?,
            tags: vec!["tag2".to_string()],
            title: "title2".to_string(),
        };
        let updated = entry.update(content2.clone(), meta2.clone());
        assert_eq!(updated.content(), content2.as_str());
        assert_eq!(updated.meta(), &meta2);
        Ok(())
    }
}
