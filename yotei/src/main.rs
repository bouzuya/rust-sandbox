mod client;
mod command;

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    Create(self::command::create::Args),
    List(self::command::list::Args),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = <Cli as clap::Parser>::parse();
    match cli.subcommand {
        Subcommand::Create(args) => self::command::create::execute(args).await,
        Subcommand::List(args) => self::command::list::execute(args).await,
    }
}
