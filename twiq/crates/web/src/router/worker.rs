use std::sync::Arc;

use axum::{response::IntoResponse, routing, Extension, Router};
use use_case::command::{create_user_request, send_user_request, update_user};

pub(crate) fn router<T>() -> Router
where
    T: create_user_request::Has + send_user_request::Has + update_user::Has + Send + Sync + 'static,
{
    Router::new()
        .route(
            "/_workers/create_user_request",
            routing::post(create_user_request::<T>),
        )
        .route(
            "/_workers/send_user_request",
            routing::post(send_user_request::<T>),
        )
        .route("/_workers/update_user", routing::post(update_user::<T>))
}

async fn create_user_request<T>(Extension(application): Extension<Arc<T>>) -> impl IntoResponse
where
    T: create_user_request::Has + Send + Sync,
{
    let _ = application
        .create_user_request(create_user_request::Command)
        .await;
    ""
}

async fn send_user_request<T>(Extension(application): Extension<Arc<T>>) -> impl IntoResponse
where
    T: send_user_request::Has + Send + Sync,
{
    let _ = application
        .send_user_request(send_user_request::Command)
        .await;
    ""
}

async fn update_user<T>(Extension(application): Extension<Arc<T>>) -> impl IntoResponse
where
    T: update_user::Has + Send + Sync,
{
    let _ = application.update_user(update_user::Command).await;
    ""
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use axum::async_trait;
    use domain::{aggregate::user::TwitterUserId, event::EventType};
    use hyper::{Body, Request, StatusCode};
    use tower::ServiceExt;
    use use_case::{
        command::request_user,
        event_store::{EventStore, HasEventStore},
        in_memory_event_store::InMemoryEventStore,
        in_memory_user_repository::InMemoryUserRepository,
        in_memory_user_request_repository::InMemoryUserRequestRepository,
        in_memory_worker_repository::InMemoryWorkerRepository,
        user_repository::HasUserRepository,
        user_request_repository::HasUserRequestRepository,
        worker_repository::HasWorkerRepository,
    };

    use super::*;

    #[derive(Debug)]
    struct MockApp {
        event_store: InMemoryEventStore,
        user_repository: InMemoryUserRepository,
        user_request_repository: InMemoryUserRequestRepository,
        worker_repository: InMemoryWorkerRepository,
    }

    impl Default for MockApp {
        fn default() -> Self {
            let event_store = InMemoryEventStore::default();
            let user_repository = InMemoryUserRepository::new(event_store.clone());
            let user_request_repository = InMemoryUserRequestRepository::new(event_store.clone());
            Self {
                event_store,
                user_repository,
                user_request_repository,
                worker_repository: Default::default(),
            }
        }
    }

    impl HasEventStore for MockApp {
        type EventStore = InMemoryEventStore;

        fn event_store(&self) -> &Self::EventStore {
            &self.event_store
        }
    }

    impl HasUserRequestRepository for MockApp {
        type UserRequestRepository = InMemoryUserRequestRepository;

        fn user_request_repository(&self) -> &Self::UserRequestRepository {
            &self.user_request_repository
        }
    }

    impl HasWorkerRepository for MockApp {
        type WorkerRepository = InMemoryWorkerRepository;

        fn worker_repository(&self) -> &Self::WorkerRepository {
            &self.worker_repository
        }
    }

    #[async_trait]
    impl create_user_request::Has for MockApp {}

    impl HasUserRepository for MockApp {
        type UserRepository = InMemoryUserRepository;

        fn user_repository(&self) -> &Self::UserRepository {
            &self.user_repository
        }
    }

    #[async_trait]
    impl request_user::Has for MockApp {}

    #[async_trait]
    impl send_user_request::Has for MockApp {}

    #[async_trait]
    impl update_user::Has for MockApp {}

    #[tokio::test]
    async fn test_create_user_request_no_events() -> anyhow::Result<()> {
        let router = router::<MockApp>();
        let application = MockApp::default();
        let application = Arc::new(application);
        let router = router.layer(Extension(application));
        let (status, body) = post_request(router, "/_workers/create_user_request").await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, r#""#);
        Ok(())
    }

    #[tokio::test]
    async fn test_create_user_request() -> anyhow::Result<()> {
        let router = router::<MockApp>();
        let application = MockApp::default();
        let event_store = application.event_store.clone();

        // add UserRequested event
        let twitter_user_id = TwitterUserId::from_str("125962981")?;
        request_user::Has::request_user(
            &application,
            request_user::Command {
                twitter_user_id: twitter_user_id.clone(),
            },
        )
        .await?;

        let application = Arc::new(application);
        let router = router.layer(Extension(application.clone()));
        let (status, body) = post_request(router, "/_workers/create_user_request").await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, r#""#);

        let event_types = event_store
            .find_events(None)
            .await?
            .into_iter()
            .map(|event| EventType::try_from(event.r#type().clone()).map_err(anyhow::Error::from))
            .collect::<anyhow::Result<Vec<EventType>>>()?;
        assert_eq!(
            event_types,
            vec![
                EventType::UserCreated,
                EventType::UserRequested,
                EventType::UserRequestCreated
            ]
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_send_user_request_no_events() -> anyhow::Result<()> {
        let router = router::<MockApp>();
        let application = MockApp::default();
        let application = Arc::new(application);
        let router = router.layer(Extension(application));
        let (status, body) = post_request(router, "/_workers/send_user_request").await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, r#""#);
        Ok(())
    }

    // TODO: test_send_user_request

    #[tokio::test]
    async fn test_update_user_no_events() -> anyhow::Result<()> {
        let router = router::<MockApp>();
        let application = MockApp::default();
        let application = Arc::new(application);
        let router = router.layer(Extension(application));
        let (status, body) = post_request(router, "/_workers/update_user").await?;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, r#""#);
        Ok(())
    }

    // TODO: test_update_user

    async fn post_request(router: Router, uri: &str) -> anyhow::Result<(StatusCode, String)> {
        let request = Request::post(uri).body(Body::empty())?;
        let response = router.oneshot(request).await?;
        let status = response.status();
        let body_as_bytes = hyper::body::to_bytes(response.into_body()).await?;
        let body_as_string = String::from_utf8(Vec::<u8>::from(body_as_bytes))?;
        Ok((status, body_as_string))
    }
}
