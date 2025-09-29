#[derive(Debug, thiserror::Error)]
#[error("user reader")]
pub struct UserReaderError(#[source] pub Box<dyn std::error::Error + Send + Sync>);

#[async_trait::async_trait]
pub trait UserReader {
    async fn list(&self) -> Result<Vec<crate::query_models::QueryUser>, UserReaderError>;
}
