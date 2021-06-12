use crate::bid::BId;

#[derive(Debug, Eq, PartialEq)]
pub struct BMeta {
    pub id: BId,
    pub tags: Vec<String>,
    pub title: String,
}
