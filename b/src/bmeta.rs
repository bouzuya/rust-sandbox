use crate::bid::BId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BMeta {
    pub id: BId,
    pub tags: Vec<String>,
    pub title: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_and_eq_test() {
        let id = BId::now();
        let tags = vec!["t1".to_string()];
        let title = "title".to_string();
        let meta = BMeta { id, tags, title };
        assert_eq!(meta, meta.clone());
    }
}
