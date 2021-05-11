use std::env;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "b-pwd", about = "bouzuya's pwd: print working directory")]
struct Opt {}

fn main() {
    Opt::from_args();
    println!(
        "{}",
        env::current_dir()
            .expect("current_dir failed")
            .to_str()
            .expect("current_dir is not UTF-8")
    );
}
