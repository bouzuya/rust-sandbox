mod router;

use axum::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = router::router();
    let addr = "0.0.0.0:3000".parse()?;
    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
