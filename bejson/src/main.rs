use bejson::JsonValue;
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
    let json = content.parse::<JsonValue>().unwrap();
    println!("{}", json.eval().unwrap());
}
