mod bid;
mod list;
mod query;

use b::{build_data, list_entries, TemplateEntry};
use list::list;
use std::{convert::TryFrom, env, fs, io, path::PathBuf};
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
        } => list(data_dir, json, query, &mut io::stdout()),
        Subcommand::New {
            data_file,
            template,
        } => {
            let data = if data_file == PathBuf::from("-").as_path() {
                let stdin = io::stdin();
                let mut handle = stdin.lock();
                build_data(&mut handle)
            } else {
                build_data(&mut fs::File::open(&data_file).unwrap())
            }
            .unwrap();
            let entries = list_entries(template.as_path()).unwrap();
            let templates = entries
                .iter()
                .map(TemplateEntry::try_from)
                .collect::<Result<Vec<TemplateEntry>, _>>()
                .unwrap();
            let root_dir = env::current_dir().unwrap();
            for template in templates {
                template.render(root_dir.as_path(), &data);
            }
        }
    }
}
