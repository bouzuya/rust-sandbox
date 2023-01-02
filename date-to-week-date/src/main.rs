mod command;

use axum::{
    extract::Query, http::StatusCode, response::IntoResponse, routing::get, Router, Server,
};
use command::date_to_week_date;

async fn handle_root(Query(command): Query<date_to_week_date::Command>) -> impl IntoResponse {
    date_to_week_date::handle(command).map_err(|e| match e {
        date_to_week_date::Error::InvalidDateFormat => {
            (StatusCode::BAD_REQUEST, "Invalid `date` parameter")
        }
        date_to_week_date::Error::InvalidWeekDateFormat => {
            (StatusCode::INTERNAL_SERVER_ERROR, "week_date_format")
        }
    })
}

async fn handle_healthz() -> impl IntoResponse {
    "OK"
}

fn build_router(base_path: String) -> Router {
    let router = Router::new()
        .route("/", get(handle_root))
        .route("/healthz", get(handle_healthz));
    if base_path.is_empty() {
        router
    } else {
        Router::new()
            .route("/", get(handle_root))
            .nest(base_path.as_str(), router)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let base_path = std::env::var("BASE_PATH").unwrap_or_else(|_| "".to_string());
    let router = build_router(base_path);
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let host = "0.0.0.0";
    let addr = format!("{}:{}", host, port).parse()?;
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}
