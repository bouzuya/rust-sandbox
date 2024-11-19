use crate::AppState;

use tower_http::services::ServeDir;

pub fn route() -> axum::Router<AppState> {
    axum::Router::new().nest_service("/assets", ServeDir::new("assets"))
}
