use bejson::json;
use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    file: PathBuf,
}

fn main() {
    // TODO: tests/bejson.rs
    let opt = Opt::from_args();
    let content = fs::read_to_string(opt.file).unwrap();
    println!("{}", json(&content).unwrap().1);
}
