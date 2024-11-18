mod discovery_document;
mod handlers;
mod session;
mod session_id;
mod user;
mod user_id;
mod user_secret;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use discovery_document::DiscoveryDocument;
use session::Session;
use session_id::SessionId;
use user::User;
use user_id::UserId;

#[derive(Clone)]
pub(crate) struct AppState {
    authorization_endpoint: String,
    client_id: String,
    client_secret: String,
    sessions: Arc<Mutex<HashMap<SessionId, Session>>>,
    token_endpoint: String,
    users: Arc<Mutex<HashMap<UserId, User>>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let DiscoveryDocument {
        authorization_endpoint,
        token_endpoint,
    } = DiscoveryDocument::fetch().await?;
    let client_id = std::env::var("CLIENT_ID")?;
    let client_secret = std::env::var("CLIENT_SECRET")?;

    println!("authorization_endpoint={}", authorization_endpoint);
    println!("client_id={}", client_id);
    println!("client_secret={}", client_secret);
    println!("token_endpoint={}", token_endpoint);

    let state = AppState {
        authorization_endpoint,
        client_id,
        client_secret,
        sessions: Arc::new(Mutex::new(Default::default())),
        token_endpoint,
        users: Arc::new(Mutex::new(Default::default())),
    };
    let router = handlers::route().with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, router).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() -> anyhow::Result<()> {
        let url = url::Url::parse("https://accounts.google.com/o/oauth2/v2/auth")?;
        assert_eq!(
            url.to_string(),
            "https://accounts.google.com/o/oauth2/v2/auth"
        );
        Ok(())
    }
}
