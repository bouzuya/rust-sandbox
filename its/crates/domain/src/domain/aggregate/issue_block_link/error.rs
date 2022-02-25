use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum IssueBlockLinkAggregateError {
    #[error("Block")]
    Block,
    #[error("NextVersion")]
    NoNextVersion,
}
