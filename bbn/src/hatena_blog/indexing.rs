use crate::hatena_blog::IndexingId;
use crate::timestamp::Timestamp;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Indexing {
    id: IndexingId,
    at: Timestamp,
}

impl Indexing {
    pub fn new(id: IndexingId, at: Timestamp) -> Self {
        Self { id, at }
    }

    pub fn at(&self) -> Timestamp {
        self.at
    }

    pub fn id(&self) -> IndexingId {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let id = IndexingId::from(1_i64);
        let at = Timestamp::now()?;
        let indexing = Indexing::new(id, at);
        assert_eq!(indexing.id(), id);
        assert_eq!(indexing.at(), at);
        Ok(())
    }
}
