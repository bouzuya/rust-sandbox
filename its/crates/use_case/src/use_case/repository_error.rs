use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("IO")]
    IO,
    #[error("Unknown: {0}")]
    Unknown(String),
}
