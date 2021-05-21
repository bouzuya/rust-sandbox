use b::{list_entries, TemplateEntry};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    convert::TryFrom,
    env, fs,
    io::{self, Read},
    path::PathBuf,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
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
        Subcommand::New {
            data_file,
            template,
        } => {
            let data = {
                let data = if data_file == PathBuf::from("-") {
                    let stdin = io::stdin();
                    let mut handle = stdin.lock();
                    let mut buf = String::new();
                    handle.read_to_string(&mut buf).unwrap();
                    buf
                } else {
                    fs::read_to_string(&data_file).unwrap()
                };
                let data: Value = serde_json::from_str(&data).unwrap();
                let object = data.as_object().unwrap();
                let mut map = BTreeMap::new();
                for (k, v) in object {
                    if !k.chars().all(|c| c.is_ascii_lowercase()) {
                        panic!();
                    }
                    let v = v.as_str().unwrap().to_string();
                    map.insert(k.clone(), v);
                }
                map
            };

            let entries = list_entries(template.as_path()).unwrap();
            let templates = entries
                .iter()
                .map(|e| TemplateEntry::try_from(e))
                .collect::<Result<Vec<TemplateEntry>, _>>()
                .unwrap();
            let root_dir = env::current_dir().unwrap();
            for template in templates {
                template.render(root_dir.as_path(), &data);
            }
        }
    }
}
