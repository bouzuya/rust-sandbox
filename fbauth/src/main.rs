use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new().nest_service("/", ServeDir::new("public"));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, router).await?;
    Ok(())
}
