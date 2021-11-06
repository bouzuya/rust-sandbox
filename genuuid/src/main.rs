use std::io;

use structopt::{clap::Shell, StructOpt};
use uuid::Uuid;

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
    Generate,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    match opt.subcommand.unwrap_or(Subcommand::Generate) {
        Subcommand::Completion { shell } => {
            Opt::clap().gen_completions_to("genuuid", shell, &mut io::stdout());
            Ok(())
        }
        Subcommand::Generate => {
            let uuid = Uuid::new_v4();
            print!("{}", uuid);
            Ok(())
        }
    }
}
