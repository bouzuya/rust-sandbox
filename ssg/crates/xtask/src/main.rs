#[derive(clap::Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    Preview,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();
    match cli.subcommand {
        Subcommand::Preview => {
            let router = axum::Router::new().route("/", axum::routing::get("OK"));
            let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
            axum::serve(listener, router).await?;
            Ok(())
        }
    }
}
