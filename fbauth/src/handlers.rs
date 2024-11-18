mod callback;
mod create_authorization_urls;
mod create_session;
mod create_user;

use crate::AppState;

use tower_http::services::{ServeDir, ServeFile};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Claims {
    /// session_id
    sid: String,
    sub: String,
    // ...
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
        .merge(callback::route())
        .merge(create_authorization_urls::route())
        .merge(create_session::route())
        .merge(create_user::route())
        .route_service("/", ServeFile::new("assets/index.html"))
        .nest_service("/assets", ServeDir::new("assets"))
}
