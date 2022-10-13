mod router;

use std::{env, sync::Arc};

use axum::{Extension, Server};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use use_case::{
    command::{create_user_request, request_user, send_user_request, update_user},
    event_store::HasEventStore,
    in_memory_event_store::InMemoryEventStore,
    in_memory_user_repository::InMemoryUserRepository,
    in_memory_user_request_repository::InMemoryUserRequestRepository,
    in_memory_worker_repository::InMemoryWorkerRepository,
    user_repository::HasUserRepository,
    user_request_repository::HasUserRequestRepository,
    worker_repository::HasWorkerRepository,
};

struct App {
    event_store: InMemoryEventStore,
    user_repository: InMemoryUserRepository,
    user_request_repository: InMemoryUserRequestRepository,
    worker_repository: InMemoryWorkerRepository,
}

impl Default for App {
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

impl HasEventStore for App {
    type EventStore = InMemoryEventStore;

    fn event_store(&self) -> &Self::EventStore {
        &self.event_store
    }
}

impl HasUserRepository for App {
    type UserRepository = InMemoryUserRepository;

    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
}

impl HasUserRequestRepository for App {
    type UserRequestRepository = InMemoryUserRequestRepository;

    fn user_request_repository(&self) -> &Self::UserRequestRepository {
        &self.user_request_repository
    }
}

impl HasWorkerRepository for App {
    type WorkerRepository = InMemoryWorkerRepository;

    fn worker_repository(&self) -> &Self::WorkerRepository {
        &self.worker_repository
    }
}

impl request_user::Has for App {}
impl create_user_request::Has for App {}
impl send_user_request::Has for App {}
impl update_user::Has for App {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let app = App::default();
    let app = Arc::new(app);
    let app = router::router::<App>().layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(Extension(app)),
    );
    let host = "0.0.0.0";
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("{}:{}", host, port).parse()?;
    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
