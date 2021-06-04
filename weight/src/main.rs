use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs, io,
    path::{Path, PathBuf},
};
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

#[derive(Serialize, Deserialize)]
struct Set {
    key: String,
    value: f64,
}

fn read_jsonl(path: &Path) -> Vec<Set> {
    if !path.exists() {
        return vec![];
    }
    let content = fs::read_to_string(path).unwrap();
    let mut jsonl = vec![];
    for line in content.split('\n') {
        if line.is_empty() {
            continue;
        }
        let set: Set = serde_json::from_str(line).unwrap();
        jsonl.push(set);
    }
    jsonl
}

fn write_jsonl(path: &Path, jsonl: Vec<Set>) {
    let mut output = String::new();
    for set in jsonl {
        let line = serde_json::to_string(&set).unwrap();
        output.push_str(line.as_str());
        output.push('\n');
    }
    fs::write(path, output).unwrap();
}

fn main() {
    let opt = Opt::from_args();

    match opt.subcommand {
        Subcommand::Completion { shell } => {
            Opt::clap().gen_completions_to("weight", shell, &mut io::stdout())
        }
        Subcommand::List => {
            let jsonl = read_jsonl(opt.data_file.as_path());

            let state = jsonl.iter().fold(BTreeMap::new(), |mut map, e| {
                map.insert(e.key.clone(), e.value.clone());
                map
            });

            for (k, v) in state {
                println!("{} {}", k, v);
            }
        }
        Subcommand::Set { key, value } => {
            let mut jsonl = read_jsonl(opt.data_file.as_path());

            jsonl.push(Set { key, value });

            write_jsonl(opt.data_file.as_path(), jsonl);
        }
    }
}
