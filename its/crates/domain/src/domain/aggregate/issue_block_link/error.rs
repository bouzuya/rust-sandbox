#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("Block")]
    Block,
    #[error("InvalidEventSequence")]
    InvalidEventSequence,
    #[error("NextVersion")]
    NoNextVersion,
}
