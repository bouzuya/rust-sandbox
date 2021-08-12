mod get;
mod root;

use self::get::get;
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
    #[structopt(name = "root", about = "Prints root")]
    Root,
}

pub fn run() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    match opt.subcommand {
        Subcommand::Get { name } => {
            get(name)?;
        }
        Subcommand::Root => {
            println!("{:?}", root()?);
        }
    }
    Ok(())
}
