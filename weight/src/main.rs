mod jsonl;
mod set;

use crate::jsonl::{read_jsonl, write_jsonl};
use crate::set::Set;
use std::{collections::BTreeMap, io, path::PathBuf};
use structopt::{clap::Shell, StructOpt};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long = "data-file", default_value = "weight.jsonl")]
    data_file: PathBuf,
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
    List,
    Set {
        key: String,
        value: f64,
    },
}
fn main() {
    let opt = Opt::from_args();

    match opt.subcommand {
        Subcommand::Completion { shell } => {
            Opt::clap().gen_completions_to("weight", shell, &mut io::stdout())
        }
        Subcommand::List => {
            let events = read_jsonl(opt.data_file.as_path());
            let state = events.iter().fold(BTreeMap::new(), |mut map, e| {
                map.insert(e.key.clone(), e.value.clone());
                map
            });
            for (k, v) in state {
                println!("{} {}", k, v);
            }
        }
        Subcommand::Set { key, value } => {
            let mut events = read_jsonl(opt.data_file.as_path());
            events.push(Set { key, value });
            write_jsonl(opt.data_file.as_path(), events);
        }
    }
}
