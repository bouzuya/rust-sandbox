mod get;
mod list;
mod root;

use self::get::get;
use self::list::list;
use self::root::root;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(name = "get", about = "Clone a repository")]
    Get {
        #[structopt(name = "USER/REPO")]
        name: String,
    },
    #[structopt(name = "list", about = "List repositories")]
    List {
        #[structopt(long = "full-path")]
        full_path: bool,
        #[structopt(name = "QUERY")]
        query: Option<String>,
    },
    #[structopt(name = "root", about = "Prints root")]
    Root,
}

pub fn run() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    match opt.subcommand {
        Subcommand::Get { name } => {
            get(name)?;
        }
        Subcommand::List { full_path, query } => {
            list(query, full_path)?;
        }
        Subcommand::Root => {
            println!("{}", root()?.display());
        }
    }
    Ok(())
}
