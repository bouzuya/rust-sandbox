use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, io, path::PathBuf};
use structopt::{clap::Shell, StructOpt};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long = "data-file")]
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

fn main() {
    let opt = Opt::from_args();
    if !opt.data_file.exists() {
        fs::write(opt.data_file.as_path(), "").unwrap();
    }
    let content = fs::read_to_string(opt.data_file.as_path()).unwrap();
    let mut json = vec![];
    for line in content.split('\n') {
        if line.is_empty() {
            continue;
        }
        let set: Set = serde_json::from_str(line).unwrap();
        json.push(set);
    }
    let state = json.iter().fold(BTreeMap::new(), |mut map, e| {
        map.insert(e.key.clone(), e.value.clone());
        map
    });

    match opt.subcommand {
        Subcommand::Completion { shell } => {
            Opt::clap().gen_completions_to("weight", shell, &mut io::stdout())
        }
        Subcommand::List => {
            for (k, v) in state {
                println!("{} {}", k, v);
            }
        }
        Subcommand::Set { key, value } => {
            json.push(Set { key, value });
        }
    }

    let mut output = String::new();
    for set in json {
        let line = serde_json::to_string(&set).unwrap();
        output.push_str(line.as_str());
        output.push('\n');
    }
    fs::write(opt.data_file.as_path(), output).unwrap();
}
