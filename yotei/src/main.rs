mod client;
mod command;
mod config;

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    Create(self::command::create::Args),
    Delete(self::command::delete::Args),
    Get(self::command::get::Args),
    List(self::command::list::Args),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();
    match cli.subcommand {
        Subcommand::Create(args) => self::command::create::execute(args).await,
        Subcommand::Delete(args) => self::command::delete::execute(args).await,
        Subcommand::Get(args) => self::command::get::execute(args).await,
        Subcommand::List(args) => self::command::list::execute(args).await,
    }
}
