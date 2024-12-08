use crate::AppState;

async fn handle() -> axum::response::Html<&'static str> {
    axum::response::Html(include_str!("../../assets/index.html"))
}

pub fn route() -> axum::Router<AppState> {
    axum::Router::new().route("/", axum::routing::get(handle))
}
