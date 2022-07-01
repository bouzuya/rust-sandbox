#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("InvalidEventSequence")]
    InvalidEventSequence,
    #[error("Unknown")]
    Unknown,
}
