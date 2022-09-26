use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing, Extension, Router};
use use_case::command::request_user;

pub(crate) fn router<T>() -> Router
where
    T: request_user::Has + Send + Sync + 'static,
{
    Router::new().route("/users/:id", routing::get(users_show::<T>))
}

async fn users_show<T>(
    Extension(application): Extension<Arc<T>>,
    Path(id): Path<String>,
) -> impl IntoResponse
where
    T: request_user::Has + Send + Sync,
{
    // TODO: if the user is cached, return it; otherwise, enqueue the ID.
    // TODO: error handling
    // ignore errors
    let _ = application
        .request_user(request_user::Command {
            twitter_user_id: id.parse().unwrap(),
        })
        .await;
    (StatusCode::ACCEPTED, id)
}

#[cfg(test)]
mod tests {
    use axum::async_trait;
    use db::in_memory_user_repository::InMemoryUserRepository;
    use hyper::StatusCode;
    use use_case::user_repository::HasUserRepository;

    use crate::router::tests::test_get_request;

    use super::*;

    struct MockApp {
        user_repository: InMemoryUserRepository,
    }

    impl HasUserRepository for MockApp {
        type UserRepository = InMemoryUserRepository;

        fn user_repository(&self) -> &Self::UserRepository {
            &self.user_repository
        }
    }

    #[async_trait]
    impl request_user::Has for MockApp {}

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let router = router::<MockApp>();
        let application = MockApp {
            user_repository: InMemoryUserRepository::default(),
        };
        let application = Arc::new(application);
        let router = router.layer(Extension(application));
        let (status, body) = test_get_request(router, "/users/125962981").await?;
        assert_eq!(status, StatusCode::ACCEPTED);
        assert_eq!(body, r#"125962981"#);
        Ok(())
    }
}
