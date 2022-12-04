mod domain;
mod import;

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, clap::Subcommand)]
enum Subcommand {
    Import { file: String },
}

fn main() -> anyhow::Result<()> {
    let args = <Args as clap::Parser>::parse();
    match args.subcommand {
        Subcommand::Import { file } => {
            import::import(file)?;
        }
    }
    Ok(())
}
