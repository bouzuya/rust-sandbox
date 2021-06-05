mod jsonl;
mod set;
mod sqlite;

use crate::jsonl::{read_jsonl, write_jsonl};
use crate::set::Set;
use sqlite::{read_sqlite, write_sqlite};
use std::{collections::BTreeMap, io, path::PathBuf};
use structopt::{clap::Shell, StructOpt};

#[derive(Debug)]
enum DataFileType {
    Jsonl,
    Sqlite,
}

impl std::str::FromStr for DataFileType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "jsonl" => DataFileType::Jsonl,
            "sqlite" => DataFileType::Sqlite,
            _ => return Err("unknown data file type"),
        })
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long = "data-file-type", name = "DATA_FILE_TYPE", default_value = "jsonl", possible_values = &["jsonl", "sqlite"])]
    data_file_type: DataFileType,
    #[structopt(long = "data-file", name = "DATA_FILE", default_value = "weight.jsonl")]
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

#[async_std::main]
async fn main() {
    let opt = Opt::from_args();

    match opt.subcommand {
        Subcommand::Completion { shell } => {
            Opt::clap().gen_completions_to("weight", shell, &mut io::stdout())
        }
        Subcommand::List => {
            let events = match opt.data_file_type {
                DataFileType::Jsonl => read_jsonl(opt.data_file.as_path()).unwrap(),
                DataFileType::Sqlite => read_sqlite(opt.data_file.as_path()).await.unwrap(),
            };
            let state = events.iter().fold(BTreeMap::new(), |mut map, e| {
                map.insert(e.key(), e.value());
                map
            });
            for (k, v) in state {
                println!("{} {}", k, v);
            }
        }
        Subcommand::Set { key, value } => {
            let mut events = match opt.data_file_type {
                DataFileType::Jsonl => read_jsonl(opt.data_file.as_path()).unwrap(),
                DataFileType::Sqlite => read_sqlite(opt.data_file.as_path()).await.unwrap(),
            };
            events.push(Set::new(key, value).unwrap());
            match opt.data_file_type {
                DataFileType::Jsonl => write_jsonl(opt.data_file.as_path(), &events).unwrap(),
                DataFileType::Sqlite => write_sqlite(opt.data_file.as_path(), &events)
                    .await
                    .unwrap(),
            };
        }
    }
}
