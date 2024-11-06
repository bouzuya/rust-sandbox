mod discovery_document;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Query, State},
    response::Html,
    routing::{get, post},
    Json, Router,
};
use discovery_document::DiscoveryDocument;
use tower_http::services::ServeDir;

#[derive(serde::Deserialize)]
struct CallbackQueryParams {
    // authuser: String,
    code: String,
    // hd: String,
    // prompt: String
    // scope: String,
    state: String,
}

#[derive(serde::Serialize)]
struct TokenRequestBody {
    code: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    grant_type: String,
}

async fn callback(
    State(app_state): State<AppState>,
    Query(query): Query<CallbackQueryParams>,
) -> Html<String> {
    // FIXME: check state
    println!("query.state = {}", query.state);
    let redirect_uri = "http://localhost:3000/callback".to_owned();

    let response = reqwest::Client::new()
        .post(app_state.token_endpoint)
        .json(&TokenRequestBody {
            code: query.code,
            client_id: app_state.client_id,
            client_secret: app_state.client_secret,
            redirect_uri,
            grant_type: "authorization_code".to_owned(),
        })
        .send()
        .await
        .unwrap();
    if !response.status().is_success() {
        println!("status code = {}", response.status());
        println!("response body = {}", response.text().await.unwrap());
        Html("ERROR".to_owned())
    } else {
        let response_body = response.text().await.unwrap();
        // let body = serde_json::from_str(&response_body).unwrap();
        Html(response_body)
    }
}

#[derive(serde::Serialize)]
struct CreateUserResponse {
    user_id: String,
    user_secret: String,
}

async fn create_user(State(app_state): State<AppState>) -> Json<CreateUserResponse> {
    let mut users = app_state.users.lock().unwrap();
    let user = User::new();
    users.insert(user.id, user.clone());
    Json(CreateUserResponse {
        user_id: user.id.to_string(),
        user_secret: user.secret.to_string(),
    })
}

async fn root(State(app_state): State<AppState>) -> Html<String> {
    let client_id = &app_state.client_id;
    let nonce = "FIXME";
    let redirect_uri = "http://localhost:3000/callback";
    let state = "FIXME";

    let mut url = url::Url::parse(&app_state.authorization_endpoint).unwrap();
    url.query_pairs_mut()
        .clear()
        .append_pair("response_type", "code")
        .append_pair("client_id", client_id)
        .append_pair("scope", "openid email")
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("state", state)
        .append_pair("nonce", nonce);
    let authorization_url = url.to_string();

    Html(format!(
        r#"<html>
  <head>
    <title>Title</title>
  </head>
  <body>
    <p><a href="{}">Login</a></p>
  </body
</html>"#,
        authorization_url
    ))
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct UserId(uuid::Uuid);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Eq, PartialEq)]
struct UserSecret(uuid::Uuid);

impl std::fmt::Display for UserSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Eq, PartialEq)]
struct User {
    id: UserId,
    secret: UserSecret,
}

impl User {
    fn new() -> Self {
        Self {
            id: UserId(uuid::Uuid::new_v4()),
            secret: UserSecret(uuid::Uuid::new_v4()),
        }
    }
}

#[derive(Clone)]
struct AppState {
    authorization_endpoint: String,
    client_id: String,
    client_secret: String,
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

    let router = Router::new()
        .route("/", get(root))
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/callback", get(callback))
        .route("/users", post(create_user))
        .with_state(AppState {
            authorization_endpoint,
            client_id,
            client_secret,
            token_endpoint,
            users: Arc::new(Mutex::new(Default::default())),
        });
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
