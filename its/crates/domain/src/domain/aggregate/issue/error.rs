use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum IssueAggregateError {
    #[error("InvalidEventSequence")]
    InvalidEventSequence,
    #[error("Unknown")]
    Unknown,
}
