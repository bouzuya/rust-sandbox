mod assets;
mod callback;
mod create_authorization_urls;
mod create_session;
mod create_user;
mod root;

use crate::AppState;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Claims {
    exp: u64,
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
        .merge(assets::route())
        .merge(callback::route())
        .merge(create_authorization_urls::route())
        .merge(create_session::route())
        .merge(create_user::route())
        .merge(root::route())
}
