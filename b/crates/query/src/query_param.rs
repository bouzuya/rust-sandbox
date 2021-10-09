use crate::{DateParam, TagParam};

use thiserror::Error;

#[derive(Debug, Error)]
#[error("parse query error")]
pub struct ParseQueryError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryParam {
    Date(DateParam),
    Tag(TagParam),
}

impl std::fmt::Display for QueryParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                QueryParam::Date(d) => d.to_string(),
                QueryParam::Tag(t) => t.to_string(),
            }
        )
    }
}
