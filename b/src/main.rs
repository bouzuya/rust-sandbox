use b::{use_case, BId};
use std::{io, path::PathBuf};
use structopt::{clap::Shell, StructOpt};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(name = "completion", about = "Prints the shell's completion script")]
    Completion {
        #[structopt(name = "SHELL", help = "the shell", possible_values = &Shell::variants())]
        shell: Shell,
    },
    #[structopt(name = "list", about = "Lists b files")]
    List {
        #[structopt(long = "data-dir")]
        data_dir: PathBuf,
        #[structopt(long = "json")]
        json: bool,
        query: String,
        #[structopt(long = "time-zone-offset")]
        time_zone_offset: Option<String>,
    },
    #[structopt(name = "new", about = "Creates a new file")]
    New {
        #[structopt(short = "d", long = "data-file", help = "The data file")]
        data_file: PathBuf,
        #[structopt(
            short = "t",
            long = "template",
            help = "The template file or directory"
        )]
        template: PathBuf,
    },
    #[structopt(name = "view", about = "Views the b file")]
    View {
        #[structopt(long = "data-dir")]
        data_dir: PathBuf,
        #[structopt(name = "BID")]
        id: BId,
    },
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    match opt.subcommand {
        Subcommand::Completion { shell } => {
            Opt::clap().gen_completions_to("b", shell, &mut io::stdout());
            Ok(())
        }
        Subcommand::List {
            data_dir,
            json,
            query,
            time_zone_offset,
        } => use_case::list(data_dir, json, query, time_zone_offset, &mut io::stdout()),
        Subcommand::New {
            data_file,
            template,
        } => use_case::new(data_file, template),
        Subcommand::View { data_dir, id } => use_case::view(data_dir, id, &mut io::stdout()),
    }
}
