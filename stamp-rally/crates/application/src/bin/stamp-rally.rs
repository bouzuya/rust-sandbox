use adapter_console::run;
use use_case::{HasStampRallyRepository, InMemoryStampRallyRepository};

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

fn main() -> anyhow::Result<()> {
    let application = Application::new();
    run(application)
}
