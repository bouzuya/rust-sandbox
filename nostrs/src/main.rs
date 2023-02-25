mod client;
mod config;
mod contact;
mod dirs;
mod handler;
mod keypair;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Command {
    #[command(subcommand)]
    resource: Resource,
}

#[derive(clap::Subcommand)]
enum Resource {
    Contact {
        #[command(subcommand)]
        command: ContactCommand,
    },
    Keypair {
        #[command(subcommand)]
        command: KeypairCommand,
    },
    Metadata,
    TextNote {
        #[command(subcommand)]
        command: TextNoteCommand,
    },
    Timeline,
}

#[derive(clap::Subcommand)]
enum ContactCommand {
    List,
}

#[derive(clap::Subcommand)]
enum KeypairCommand {
    Create {
        #[arg(long, env)]
        private_key: String,
    },
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
        Resource::Contact { command } => match command {
            ContactCommand::List => handler::contact::list().await,
        },
        Resource::Keypair { command } => match command {
            KeypairCommand::Create { private_key } => handler::keypair::create(private_key).await,
        },
        Resource::Metadata => handler::metadata::handle().await,
        Resource::TextNote { command } => match command {
            TextNoteCommand::Create { content } => handler::text_note::create(content).await,
            TextNoteCommand::List => handler::text_note::list().await,
        },
        Resource::Timeline => handler::timeline::handle().await,
    }
}
