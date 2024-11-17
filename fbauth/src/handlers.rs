use std::{
    collections::HashMap,
    str::FromStr as _,
    sync::{Arc, Mutex},
};

use crate::session::Session;
use crate::session_id::SessionId;
use crate::user::User;
use crate::user_id::UserId;
use crate::user_secret::UserSecret;
use axum::{
    extract::{Query, State},
    http::HeaderMap,
    response::Html,
    routing::{get, post},
    Json,
};
use tower_http::services::{ServeDir, ServeFile};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    /// session_id
    sid: String,
    sub: String,
    // ...
}

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
struct CreateAuthorizationUrlResponseBody {
    authorization_url: String,
}

async fn create_authorization_url(
    header_map: HeaderMap,
    State(app_state): State<AppState>,
) -> Json<CreateAuthorizationUrlResponseBody> {
    let jwt = header_map
        .get("authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .strip_prefix("Bearer ")
        .unwrap();

    // decode jwt
    let decoding_key = jsonwebtoken::DecodingKey::from_rsa_pem(include_bytes!("../key.pem"))
        .map_err(|_| reqwest::StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();
    let jsonwebtoken::TokenData { header: _, claims } = jsonwebtoken::decode::<Claims>(
        jwt,
        &decoding_key,
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256),
    )
    .unwrap();

    // get session_id
    let mut sessions = app_state
        .sessions
        .lock()
        .map_err(|_| reqwest::StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();
    let session_id = SessionId::from_str(&claims.sid)
        .map_err(|_| reqwest::StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();

    // generate state
    let state = "FIXME";

    // set state to session
    sessions.entry(session_id).and_modify(|session| {
        session.state = Some(state.to_owned());
    });

    println!("jwt = {}", jwt);

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
    Json(CreateAuthorizationUrlResponseBody { authorization_url })
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("client")]
    Client,
    #[error("server")]
    Server,
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        axum::response::IntoResponse::into_response(match self {
            Error::Client => reqwest::StatusCode::BAD_REQUEST,
            Error::Server => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        })
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        Error::Server
    }
}

#[derive(serde::Deserialize)]
struct CreateSessionRequestBody {
    user_id: String,
    user_secret: String,
}

#[derive(serde::Serialize)]
struct CreateSessionResponse {
    session_token: String,
}

async fn create_session(
    State(app_state): State<AppState>,
    Json(CreateSessionRequestBody {
        user_id,
        user_secret,
    }): Json<CreateSessionRequestBody>,
) -> Result<Json<CreateSessionResponse>, Error> {
    let users = app_state.users.lock()?;
    let user_id = UserId::from_str(&user_id).map_err(|_| Error::Client)?;
    let user_secret = UserSecret::from_str(&user_secret).map_err(|_| Error::Client)?;
    let user = users.get(&user_id).ok_or_else(|| Error::Client)?;
    if user.secret != user_secret {
        return Err(Error::Client);
    }

    let mut sessions = app_state.sessions.lock().map_err(|_| Error::Server)?;
    let session_id = SessionId::generate();
    sessions.insert(
        session_id,
        Session {
            id: session_id,
            user_id,
            state: None,
        },
    );

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256),
        &Claims {
            sid: session_id.to_string(),
            sub: user_id.to_string(),
        },
        &jsonwebtoken::EncodingKey::from_rsa_pem(include_bytes!("../key.pem"))
            .map_err(|_| Error::Server)?,
    )
    .map_err(|_| Error::Server)?;
    Ok(Json(CreateSessionResponse {
        session_token: token.to_string(),
    }))
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

#[derive(Clone)]
struct AppState {
    authorization_endpoint: String,
    client_id: String,
    client_secret: String,
    sessions: Arc<Mutex<HashMap<SessionId, Session>>>,
    token_endpoint: String,
    users: Arc<Mutex<HashMap<UserId, User>>>,
}

pub fn route(
    authorization_endpoint: String,
    token_endpoint: String,
    client_id: String,
    client_secret: String,
) -> axum::Router {
    let router = axum::Router::new()
        .route_service("/", ServeFile::new("assets/index.html"))
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/authorization_urls", post(create_authorization_url))
        .route("/callback", get(callback))
        .route("/sessions", post(create_session))
        .route("/users", post(create_user))
        .with_state(AppState {
            authorization_endpoint,
            client_id,
            client_secret,
            sessions: Arc::new(Mutex::new(Default::default())),
            token_endpoint,
            users: Arc::new(Mutex::new(Default::default())),
        });
    router
}
