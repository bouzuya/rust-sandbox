mod assets;
mod create_authorization_urls;
mod create_session;
mod create_user;
mod get_user;
mod root;
mod sign_in;
mod sign_up;

use crate::AppState;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Claims {
    pub(crate) exp: u64,
    /// session_id
    pub(crate) sid: String,
    pub(crate) sub: String,
    // ...
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("client")]
    Client(#[source] anyhow::Error),
    #[error("server")]
    Server(#[source] anyhow::Error),
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        axum::response::IntoResponse::into_response(match self {
            Error::Client(_) => reqwest::StatusCode::BAD_REQUEST,
            Error::Server(_) => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        })
    }
}

pub fn route() -> axum::Router<AppState> {
    axum::Router::new()
        .merge(assets::route())
        .merge(create_authorization_urls::route())
        .merge(create_session::route())
        .merge(create_user::route())
        .merge(get_user::route())
        .merge(root::route())
        .merge(sign_in::route())
        .merge(sign_up::route())
}
