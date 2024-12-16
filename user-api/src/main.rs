mod handlers;
mod models;

use std::{collections::BTreeMap, sync::Arc};

use models::{user::User, user_id::UserId};
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;

#[derive(Clone, Default)]
pub struct AppState {
    users: Arc<Mutex<BTreeMap<UserId, User>>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let state = AppState::default();
    let router = handlers::route()
        .with_state(state)
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, router).await?;
    Ok(())
}
