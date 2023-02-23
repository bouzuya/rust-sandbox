mod handler;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Command {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    Metadata,
    TextNote,
    Timeline,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let command = <Command as clap::Parser>::parse();
    match command.subcommand {
        Subcommand::Metadata => handler::metadata::handle().await,
        Subcommand::TextNote => handler::text_note::handle().await,
        Subcommand::Timeline => handler::timeline::handle().await,
    }
}
