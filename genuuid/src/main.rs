mod command;
mod count;

use clap::CommandFactory;
use clap_complete::{generate, Shell};
use count::Count;
use std::io;

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Opt {
    #[command(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(Debug, clap::Subcommand)]
enum Subcommand {
    #[command(name = "completion", about = "Prints the shell's completion script")]
    Completion {
        #[arg(name = "SHELL", help = "the shell", value_enum)]
        shell: Shell,
    },
    #[command(name = "generate", about = "Generates UUID")]
    Generate {
        #[arg(long = "count", help = "the count")]
        count: Option<Count>,
    },
    #[command(name = "server", about = "Runs server")]
    Server,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = <Opt as clap::Parser>::parse();
    match opt
        .subcommand
        .unwrap_or(Subcommand::Generate { count: None })
    {
        Subcommand::Completion { shell } => {
            let mut command = <Opt as CommandFactory>::command();
            generate(shell, &mut command, "genuuid", &mut io::stdout());
            Ok(())
        }
        Subcommand::Generate { count } => command::generate(count),
        Subcommand::Server => command::server().await,
    }
}
