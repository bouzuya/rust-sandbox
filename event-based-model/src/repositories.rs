mod user_repository;

pub use self::user_repository::UserRepository;
pub use self::user_repository::UserRepositoryError;

#[cfg(test)]
mod tests {
    #[test]
    fn test_exports_user_repository() {
        #[allow(dead_code)]
        struct UserRepositoryImpl;

        #[async_trait::async_trait]
        impl crate::repositories::UserRepository for UserRepositoryImpl {
            async fn find(
                &self,
                _id: &crate::value_objects::UserId,
            ) -> Result<Option<crate::aggregates::User>, crate::repositories::UserRepositoryError>
            {
                Ok(None)
            }

            async fn store(
                &self,
                _version: Option<crate::value_objects::Version>,
                _user_events: Vec<crate::event::UserEvent>,
            ) -> Result<(), crate::repositories::UserRepositoryError> {
                Ok(())
            }
        }
    }
}
