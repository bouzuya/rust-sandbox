#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("InvalidEventSequence")]
    InvalidEventSequence,
    #[error("Unknown")]
    Unknown,
}
