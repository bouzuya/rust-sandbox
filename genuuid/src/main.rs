use structopt::StructOpt;
use uuid::Uuid;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(name = "generate", about = "Generates UUID")]
    Generate,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    match opt.subcommand.unwrap_or(Subcommand::Generate) {
        Subcommand::Generate => {
            let uuid = Uuid::new_v4();
            print!("{}", uuid);
            Ok(())
        }
    }
}
