use adapter_console::run;
use use_case::{
    HasCreateStampRallyUseCase, HasCreateUserUseCase, HasStampRallyRepository, HasUserRepository,
    InMemoryStampRallyRepository, InMemoryUserRepository,
};

struct Application {
    stamp_rally_repository: InMemoryStampRallyRepository,
    user_repository: InMemoryUserRepository,
}

impl Application {
    fn new() -> Self {
        Self {
            stamp_rally_repository: InMemoryStampRallyRepository::new(),
            user_repository: InMemoryUserRepository::new(),
        }
    }
}

// port

impl HasStampRallyRepository for Application {
    type StampRallyRepository = InMemoryStampRallyRepository;

    fn stamp_rally_repository(&self) -> &Self::StampRallyRepository {
        &self.stamp_rally_repository
    }
}

impl HasUserRepository for Application {
    type UserRepository = InMemoryUserRepository;

    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
}

// use_case

impl HasCreateStampRallyUseCase for Application {
    type CreateStampRallyUseCase = Application;

    fn create_stamp_rally_use_case(&self) -> &Self::CreateStampRallyUseCase {
        self
    }
}

impl HasCreateUserUseCase for Application {
    type CreateUserUseCase = Application;

    fn create_user_use_case(&self) -> &Self::CreateUserUseCase {
        self
    }
}

fn main() -> anyhow::Result<()> {
    let application = Application::new();
    run(application)
}
