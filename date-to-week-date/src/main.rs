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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(handle_root))
        .route("/healthz", get(handle_healthz));

    let addr = "0.0.0.0:3000".parse()?;

    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
