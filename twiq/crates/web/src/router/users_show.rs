use std::{str::FromStr, sync::Arc};

use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing, Extension, Router};
use domain::aggregate::user::TwitterUserId;
use query_handler::user_store::{HasUserStore, UserStore};
use use_case::command::request_user;

pub(crate) fn router<T>() -> Router
where
    T: request_user::Has + HasUserStore + Send + Sync + 'static,
{
    Router::new().route("/users/:id", routing::get(users_show::<T>))
}

async fn users_show<T>(
    Extension(application): Extension<Arc<T>>,
    Path(id): Path<String>,
) -> impl IntoResponse
where
    T: request_user::Has + HasUserStore + Send + Sync,
{
    let twitter_user_id = TwitterUserId::from_str(id.as_str()).unwrap();
    let user = application
        .user_store()
        .find_by_twitter_user_id(&id)
        .await
        .unwrap();
    // ignore errors
    let _ = application
        .request_user(request_user::Command { twitter_user_id })
        .await;
    match user {
        None => (StatusCode::ACCEPTED, id),
        Some(cached) => (StatusCode::OK, cached.twitter_user_name),
    }
}

#[cfg(test)]
mod tests {
    use axum::async_trait;
    use hyper::StatusCode;
    use query_handler::in_memory_user_store::InMemoryUserStore;
    use use_case::{
        in_memory_user_repository::InMemoryUserRepository, user_repository::HasUserRepository,
    };

    use crate::router::tests::test_get_request;

    use super::*;

    struct MockApp {
        user_repository: InMemoryUserRepository,
        user_store: InMemoryUserStore,
    }

    impl HasUserRepository for MockApp {
        type UserRepository = InMemoryUserRepository;

        fn user_repository(&self) -> &Self::UserRepository {
            &self.user_repository
        }
    }

    #[async_trait]
    impl request_user::Has for MockApp {}

    impl HasUserStore for MockApp {
        type UserStore = InMemoryUserStore;

        fn user_store(&self) -> &Self::UserStore {
            &self.user_store
        }
    }

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let router = router::<MockApp>();
        let application = MockApp {
            user_repository: InMemoryUserRepository::default(),
            user_store: InMemoryUserStore::default(),
        };
        let application = Arc::new(application);
        let router = router.layer(Extension(application));
        let (status, body) = test_get_request(router, "/users/125962981").await?;
        assert_eq!(status, StatusCode::ACCEPTED);
        assert_eq!(body, r#"125962981"#);
        Ok(())
    }

    #[tokio::test]
    async fn test_cached_user_exists() -> anyhow::Result<()> {
        let router = router::<MockApp>();
        let application = MockApp {
            user_repository: InMemoryUserRepository::default(),
            user_store: InMemoryUserStore::default(),
        };
        application
            .user_store
            .store(
                None,
                query_handler::user::User {
                    user_id: "40141a64-8236-48d0-958a-3cf812396ffe".to_owned(),
                    twitter_user_id: "125962981".to_owned(),
                    twitter_user_name: "bouzuya".to_owned(),
                },
            )
            .await?;
        let application = Arc::new(application);
        let router = router.layer(Extension(application));
        let (status, body) = test_get_request(router, "/users/125962981").await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, r#"bouzuya"#);
        Ok(())
    }
}
