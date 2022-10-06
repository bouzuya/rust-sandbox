mod router;

use std::{env, sync::Arc};

use axum::{Extension, Server};
use db::in_memory_user_repository::InMemoryUserRepository;
use use_case::{command::request_user, user_repository::HasUserRepository};

#[derive(Default)]
struct App {
    user_repository: InMemoryUserRepository,
}

impl HasUserRepository for App {
    type UserRepository = InMemoryUserRepository;

    fn user_repository(&self) -> &Self::UserRepository {
        &self.user_repository
    }
}

impl request_user::Has for App {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = App::default();
    let app = Arc::new(app);
    let app = router::router::<App>().layer(Extension(app));
    let host = "0.0.0.0";
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("{}:{}", host, port).parse()?;
    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
