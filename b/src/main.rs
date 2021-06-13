use b::use_case;
use std::{io, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
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
}

fn main() {
    let opt = Opt::from_args();
    match opt.subcommand {
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
    }
}
