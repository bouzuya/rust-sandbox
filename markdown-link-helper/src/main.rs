use std::fs;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "markdown-link-helper", about = "markdown link helper")]
struct Opt {
    #[structopt(name = "FILE", help = "The markdown file")]
    file: String,
}

fn main() {
    let opt = Opt::from_args();
    let content = fs::read_to_string(&opt.file).unwrap();
    let rules = markdown_link_helper::build_rules();
    markdown_link_helper::run(&rules, &content);
}
