use markdown_link_helper::{build_rules, run};
use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "markdown-link-helper", about = "markdown link helper")]
struct Opt {
    #[structopt(long = "rule-file", help = "The rule file")]
    rule_file: PathBuf,
    #[structopt(name = "FILE", help = "The markdown file")]
    file: String,
}

fn main() {
    let opt = Opt::from_args();
    let content = fs::read_to_string(&opt.file).unwrap();
    let rules = build_rules(&opt.rule_file).unwrap();
    run(&rules, &content);
}
