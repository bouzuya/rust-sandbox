use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Error)]
pub enum IssueBlockLinkAggregateError {
    #[error("Block")]
    Block,
    #[error("InvalidEventSequence")]
    InvalidEventSequence,
    #[error("NextVersion")]
    NoNextVersion,
}
