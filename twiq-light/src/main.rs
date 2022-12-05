mod domain;
mod fetch;
mod import;

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, clap::Subcommand)]
enum Subcommand {
    Fetch,
    Import { file: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = <Args as clap::Parser>::parse();
    match args.subcommand {
        Subcommand::Fetch => fetch::run().await,
        Subcommand::Import { file } => import::run(file).await,
    }
}
