use anyhow::{Context, Result};
use std::env;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "b-pwd", about = "bouzuya's pwd: print working directory")]
struct Opt {}

fn main() -> Result<()> {
    Opt::from_args();
    let current_dir = env::current_dir().with_context(|| "current_dir faiiled")?;
    let message = current_dir
        .to_str()
        .with_context(|| "current_dir is not UTF-8")?;
    println!("{}", message);
    Ok(())
}
