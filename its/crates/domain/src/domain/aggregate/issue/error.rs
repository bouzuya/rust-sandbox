use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum IssueAggregateError {
    #[error("Unknown")]
    Unknown,
}
