mod config;
mod page_id;
mod page_io;
mod page_meta;
mod subcommand;

use crate::config::Config;

#[derive(clap::Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: subcommand::Subcommand,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();
    cli.subcommand.execute().await
}
