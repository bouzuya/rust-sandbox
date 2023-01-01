use axum::{extract::Query, response::IntoResponse, routing::get, Router, Server};
use time::{macros::format_description, Date};

#[derive(Debug, serde::Deserialize)]
struct RootQuery {
    date: String,
}

async fn handle_root(Query(query): Query<RootQuery>) -> impl IntoResponse {
    let date_format = format_description!("[year]-[month]-[day]");
    // TODO: unwrap
    let date = Date::parse(&query.date, &date_format).unwrap();
    let week_date_format =
        format_description!("[year base:iso_week]-W[week_number repr:iso]-[weekday repr:monday]");
    // TODO: unwrap
    date.format(&week_date_format).unwrap()
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
