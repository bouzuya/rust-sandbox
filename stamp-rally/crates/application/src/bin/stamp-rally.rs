use adapter_console::run;
use use_case::{HasCreateStampRallyUseCase, HasStampRallyRepository, InMemoryStampRallyRepository};

struct Application {
    stamp_rally_repository: InMemoryStampRallyRepository,
}

impl Application {
    fn new() -> Self {
        Self {
            stamp_rally_repository: InMemoryStampRallyRepository::new(),
        }
    }
}

impl HasStampRallyRepository for Application {
    type StampRallyRepository = InMemoryStampRallyRepository;

    fn stamp_rally_repository(&self) -> &Self::StampRallyRepository {
        &self.stamp_rally_repository
    }
}

impl HasCreateStampRallyUseCase for Application {
    type CreateStampRallyUseCase = Application;

    fn create_stamp_rally_use_case(&self) -> &Self::CreateStampRallyUseCase {
        self
    }
}

fn main() -> anyhow::Result<()> {
    let application = Application::new();
    run(application)
}
