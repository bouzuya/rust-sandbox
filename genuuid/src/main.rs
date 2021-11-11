mod command;
mod count;

use std::io;

use structopt::{clap::Shell, StructOpt};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(name = "completion", about = "Prints the shell's completion script")]
    Completion {
        #[structopt(name = "SHELL", help = "the shell", possible_values = &Shell::variants())]
        shell: Shell,
    },
    #[structopt(name = "generate", about = "Generates UUID")]
    Generate {
        #[structopt(long = "count", help = "the count")]
        count: Option<usize>,
    },
    #[structopt(name = "server", about = "Runs server")]
    Server,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    match opt
        .subcommand
        .unwrap_or(Subcommand::Generate { count: None })
    {
        Subcommand::Completion { shell } => {
            Opt::clap().gen_completions_to("genuuid", shell, &mut io::stdout());
            Ok(())
        }
        Subcommand::Generate { count } => command::generate(count),
        Subcommand::Server => command::server().await,
    }
}
