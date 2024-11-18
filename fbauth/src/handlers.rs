mod create_authorization_urls;
mod create_session;
mod create_user;

use crate::AppState;
use axum::{
    extract::{Query, State},
    response::Html,
    routing::get,
};
use tower_http::services::{ServeDir, ServeFile};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Claims {
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

pub fn route() -> axum::Router<AppState> {
    axum::Router::new()
        .merge(create_authorization_urls::route())
        .merge(create_session::route())
        .merge(create_user::route())
        .route_service("/", ServeFile::new("assets/index.html"))
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/callback", get(callback))
}
