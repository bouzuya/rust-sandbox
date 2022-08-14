use axum::{routing, Router, Server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/healthz", routing::get(|| async { "OK" }));
    let addr = "0.0.0.0:3000".parse()?;
    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
