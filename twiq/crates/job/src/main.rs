use db::{
    config::Config, firestore_user_repository::FirestoreUserRepository,
    firestore_user_request_repository::FirestoreUserRequestRepository,
    firestore_user_store::FirestoreUserStore,
    firestore_worker_repository::FirestoreWorkerRepository,
};
use query_handler::user_store::HasUserStore;
use use_case::{
    user_repository::HasUserRepository, user_request_repository::HasUserRequestRepository,
};
use worker::{
    command::{
        create_user_request::{self},
        send_user_request::{self},
        update_query_user::{self},
        update_user::{self},
    },
    worker_repository::HasWorkerRepository,
};

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, clap::Subcommand)]
enum Subcommand {
    CreateUserRequest,
    SendUserRequest,
    UpdateQueryUser,
    UpdateUser,
}

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

impl HasUserStore for App {
    type UserStore = FirestoreUserStore;

    fn user_store(&self) -> &Self::UserStore {
        &self.user_store
    }
}

impl HasWorkerRepository for App {
    type WorkerRepository = FirestoreWorkerRepository;

    fn worker_repository(&self) -> &Self::WorkerRepository {
        &self.worker_repository
    }
}

impl create_user_request::Has for App {}
impl send_user_request::Has for App {}
impl update_query_user::Has for App {}
impl update_user::Has for App {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use Subcommand::*;

    let app = App::default();
    let args = <Args as clap::Parser>::parse();
    match args.subcommand {
        CreateUserRequest => {
            create_user_request::handler(&app, create_user_request::Command).await?
        }
        SendUserRequest => send_user_request::handler(&app, send_user_request::Command).await?,
        UpdateQueryUser => update_query_user::handler(&app, update_query_user::Command).await?,
        UpdateUser => update_user::handler(&app, update_user::Command).await?,
    }

    Ok(())
}
