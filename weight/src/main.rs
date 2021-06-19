mod event;
mod remove;
mod repository;
mod set;

use crate::set::Set;
use event::Event;
use remove::Remove;
use repository::{EventRepository, JsonlEventRepository, SqliteEventRepository};
use serde_json::{Map, Number, Value};
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
    List {
        #[structopt(long = "json")]
        json: bool,
    },
    Remove {
        key: String,
    },
    Set {
        key: String,
        value: f64,
    },
}

fn event_repository(data_file_type: DataFileType, data_file: PathBuf) -> Box<dyn EventRepository> {
    match data_file_type {
        DataFileType::Jsonl => Box::new(JsonlEventRepository::new(data_file)),
        DataFileType::Sqlite => Box::new(SqliteEventRepository::new(data_file)),
    }
}

#[async_std::main]
async fn main() {
    let opt = Opt::from_args();
    let repository = event_repository(opt.data_file_type, opt.data_file);
    match opt.subcommand {
        Subcommand::Completion { shell } => {
            Opt::clap().gen_completions_to("weight", shell, &mut io::stdout())
        }
        Subcommand::List { json } => {
            let events = repository.find_all().await.unwrap();
            let state = events.iter().fold(BTreeMap::new(), |mut map, event| {
                match event {
                    Event::Remove(remove) => {
                        map.remove(&remove.key());
                    }
                    Event::Set(set) => {
                        map.insert(set.key(), set.value());
                    }
                }
                map
            });
            if json {
                let mut map = Map::new();
                for (k, v) in state {
                    map.insert(k, Value::Number(Number::from_f64(v).unwrap()));
                }
                let value = Value::Object(map);
                println!("{}", serde_json::to_string(&value).unwrap());
            } else {
                for (k, v) in state {
                    println!("{} {}", k, v);
                }
            }
        }
        Subcommand::Remove { key } => {
            let mut events = repository.find_all().await.unwrap();
            events.push(Event::Remove(Remove::new(key)));
            repository.save(&events).await.unwrap();
        }
        Subcommand::Set { key, value } => {
            let mut events = repository.find_all().await.unwrap();
            events.push(Event::Set(Set::new(key, value).unwrap()));
            repository.save(&events).await.unwrap();
        }
    }
}
