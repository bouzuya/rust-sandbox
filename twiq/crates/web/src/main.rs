mod router;

use std::{env, sync::Arc};

use axum::{Extension, Server};
use db::{
    config::Config, firestore_user_repository::FirestoreUserRepository,
    firestore_user_request_repository::FirestoreUserRequestRepository,
    firestore_user_store::FirestoreUserStore,
    firestore_worker_repository::FirestoreWorkerRepository,
};
use query_handler::user_store::HasUserStore;
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};
use use_case::{
    command::request_user, user_repository::HasUserRepository,
    user_request_repository::HasUserRequestRepository,
};
use worker::{
    command::{create_user_request, send_user_request, update_query_user, update_user},
    worker_repository::HasWorkerRepository,
};

struct App {
    user_repository: FirestoreUserRepository,
    user_request_repository: FirestoreUserRequestRepository,
    user_store: FirestoreUserStore,
    worker_repository: FirestoreWorkerRepository,
}

impl Default for App {
    fn default() -> Self {
        let config = Config::load_from_env();
        let user_repository = FirestoreUserRepository::new(config.clone());
        let user_request_repository = FirestoreUserRequestRepository::new(config.clone());
        let user_store = FirestoreUserStore::new(config.clone());
        let worker_repository = FirestoreWorkerRepository::new(config);
        Self {
            user_repository,
            user_request_repository,
            user_store,
            worker_repository,
        }
    }
}

impl HasUserRepository for App {
    type UserRepository = FirestoreUserRepository;

    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
}

impl HasUserRequestRepository for App {
    type UserRequestRepository = FirestoreUserRequestRepository;

    fn user_request_repository(&self) -> &Self::UserRequestRepository {
        &self.user_request_repository
    }
}

impl HasWorkerRepository for App {
    type WorkerRepository = FirestoreWorkerRepository;

    fn worker_repository(&self) -> &Self::WorkerRepository {
        &self.worker_repository
    }
}

impl HasUserStore for App {
    type UserStore = FirestoreUserStore;

    fn user_store(&self) -> &Self::UserStore {
        &self.user_store
    }
}

impl request_user::Has for App {}
impl create_user_request::Has for App {}
impl send_user_request::Has for App {}
impl update_query_user::Has for App {}
impl update_user::Has for App {}

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
