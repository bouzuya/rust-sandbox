#[derive(Debug, thiserror::Error)]
#[error("user repository")]
pub struct UserRepositoryError(#[source] pub Box<dyn std::error::Error + Send + Sync>);

#[async_trait::async_trait]
pub trait UserRepository {
    async fn find(
        &self,
        id: &crate::value_objects::UserId,
    ) -> Result<Option<crate::aggregates::User>, UserRepositoryError>;

    async fn store(
        &self,
        version: Option<crate::value_objects::Version>,
        user_events: Vec<crate::event::UserEvent>,
    ) -> Result<(), UserRepositoryError>;
}
