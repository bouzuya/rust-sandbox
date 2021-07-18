#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IndexingId(i64);

impl From<IndexingId> for i64 {
    fn from(indexing_id: IndexingId) -> Self {
        indexing_id.0
    }
}

impl From<i64> for IndexingId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i64_conversion_test() {
        assert_eq!(i64::from(IndexingId::from(-1_i64)), -1_i64);
        assert_eq!(i64::from(IndexingId::from(0_i64)), 0_i64);
        assert_eq!(i64::from(IndexingId::from(1_i64)), 1_i64);
    }
}
