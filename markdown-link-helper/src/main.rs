use markdown_link_helper::{build_rules, run};
use std::{fs, path::PathBuf};

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Opt {
    #[arg(long = "rule-file", help = "The rule file")]
    rule_file: PathBuf,
    #[arg(name = "FILE", help = "The markdown file")]
    file: String,
}

fn main() -> anyhow::Result<()> {
    let opt = <Opt as clap::Parser>::parse();
    let content = fs::read_to_string(&opt.file)?;
    let rules = build_rules(&opt.rule_file)?;
    let results = run(&rules, &content);
    for (link, replaced) in results {
        match replaced {
            None => eprintln!("'{}' is a broken link", link),
            Some(replaced) => println!("{}", replaced),
        }
    }
    Ok(())
}
