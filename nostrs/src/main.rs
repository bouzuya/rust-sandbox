mod client;
mod config;
mod handler;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Command {
    #[command(subcommand)]
    resource: Resource,
}

#[derive(clap::Subcommand)]
enum Resource {
    Metadata,
    TextNote {
        #[command(subcommand)]
        command: TextNoteCommand,
    },
    Timeline,
}

#[derive(clap::Subcommand)]
enum TextNoteCommand {
    Create { content: String },
    List,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let command = <Command as clap::Parser>::parse();
    match command.resource {
        Resource::Metadata => handler::metadata::handle().await,
        Resource::TextNote { command } => match command {
            TextNoteCommand::Create { content } => handler::text_note::create(content).await,
            TextNoteCommand::List => handler::text_note::list().await,
        },
        Resource::Timeline => handler::timeline::handle().await,
    }
}
