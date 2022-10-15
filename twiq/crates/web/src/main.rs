mod router;

use std::{env, sync::Arc};

use axum::{Extension, Server};
use query_handler::{in_memory_user_store::InMemoryUserStore, user_store::HasUserStore};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};
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
    user_store: InMemoryUserStore,
    worker_repository: InMemoryWorkerRepository,
}

impl Default for App {
    fn default() -> Self {
        let event_store = InMemoryEventStore::default();
        let user_repository = InMemoryUserRepository::new(event_store.clone());
        let user_request_repository = InMemoryUserRequestRepository::new(event_store.clone());
        let user_store = InMemoryUserStore::default();
        Self {
            event_store,
            user_repository,
            user_request_repository,
            user_store,
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

impl HasUserStore for App {
    type UserStore = InMemoryUserStore;

    fn user_store(&self) -> &Self::UserStore {
        &self.user_store
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let app = App::default();
    let app = Arc::new(app);
    let app = router::router::<App>().layer(
        ServiceBuilder::new()
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            )
            .layer(Extension(app)),
    );
    let host = "0.0.0.0";
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("{}:{}", host, port).parse()?;
    info!("Listening on {}", addr);
    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
