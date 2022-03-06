use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum IssueBlockLinkAggregateError {
    #[error("Block")]
    Block,
    #[error("InvalidEventSequence")]
    InvalidEventSequence,
    #[error("NextVersion")]
    NoNextVersion,
}
