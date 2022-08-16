mod router;

use std::env;

use axum::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = router::router();
    let host = "0.0.0.0";
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("{}:{}", host, port).parse()?;
    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
