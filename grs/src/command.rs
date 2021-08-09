use std::{env, path::PathBuf};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(name = "root", about = "Prints root")]
    Root,
}

fn root() -> anyhow::Result<PathBuf> {
    Ok(match env::var("GRS_ROOT") {
        Ok(s) => PathBuf::from(s),
        Err(_) => {
            let home = env::var("HOME")?;
            PathBuf::from(home).join("grs")
        }
    })
}

pub fn run() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    match opt.subcommand {
        Subcommand::Root => {
            println!("{:?}", root()?);
        }
    }
    Ok(())
}
