#[derive(Debug, thiserror::Error)]
#[error("user writer")]
pub struct UserWriterError(#[source] pub Box<dyn std::error::Error + Send + Sync>);

#[async_trait::async_trait]
pub trait UserWriter {
    async fn update(&self, id: &crate::value_objects::UserId) -> Result<(), UserWriterError>;
}
