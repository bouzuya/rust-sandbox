use adapter_console::run;
use use_case::{
    HasCreateStampRallyUseCase, HasCreateUserUseCase, HasIssueStampCardUseCase,
    HasJoinStampRallyUseCase, HasPlayerRepository, HasStampCardRepository, HasStampRallyRepository,
    HasUserRepository, InMemoryPlayerRepository, InMemoryStampCardRepository,
    InMemoryStampRallyRepository, InMemoryUserRepository,
};

struct Application {
    player_repository: InMemoryPlayerRepository,
    stamp_card_repository: InMemoryStampCardRepository,
    stamp_rally_repository: InMemoryStampRallyRepository,
    user_repository: InMemoryUserRepository,
}

impl Application {
    fn new() -> Self {
        Self {
            player_repository: InMemoryPlayerRepository::new(),
            stamp_card_repository: InMemoryStampCardRepository::new(),
            stamp_rally_repository: InMemoryStampRallyRepository::new(),
            user_repository: InMemoryUserRepository::new(),
        }
    }
}

// port

impl HasPlayerRepository for Application {
    type PlayerRepository = InMemoryPlayerRepository;

    fn player_repository(&self) -> &Self::PlayerRepository {
        &self.player_repository
    }
}

impl HasStampCardRepository for Application {
    type StampCardRepository = InMemoryStampCardRepository;

    fn stamp_card_repository(&self) -> &Self::StampCardRepository {
        &self.stamp_card_repository
    }
}

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

impl HasIssueStampCardUseCase for Application {
    type IssueStampCardUseCase = Application;

    fn issue_stamp_card_use_case(&self) -> &Self::IssueStampCardUseCase {
        self
    }
}

impl HasJoinStampRallyUseCase for Application {
    type JoinStampRallyUseCase = Application;

    fn join_stamp_rally_use_case(&self) -> &Self::JoinStampRallyUseCase {
        self
    }
}

fn main() -> anyhow::Result<()> {
    let application = Application::new();
    run(application)
}
