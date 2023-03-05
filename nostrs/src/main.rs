mod client;
mod config;
mod contact;
mod dirs;
mod event_id;
mod handler;
mod keypair;
mod metadata_cache;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Command {
    #[command(subcommand)]
    resource: Resource,
}

#[derive(clap::Subcommand)]
enum Resource {
    /// Manage contacts
    Contact {
        #[command(subcommand)]
        command: ContactCommand,
    },
    /// Manage keypair
    Keypair {
        #[command(subcommand)]
        command: KeypairCommand,
    },
    /// Manage metadata
    Metadata {
        #[command(subcommand)]
        command: MetadataCommand,
    },
    /// Manage text-notes
    TextNote {
        #[command(subcommand)]
        command: TextNoteCommand,
    },
}

#[derive(clap::Subcommand)]
enum ContactCommand {
    /// List contact
    List,
}

#[derive(clap::Subcommand)]
enum KeypairCommand {
    /// Create a keypair from a private key
    Create {
        #[arg(long, env)]
        private_key: String,
    },
    /// Get the keypair
    Get {
        #[arg(value_enum, default_value_t = PrivateKeyOrPublicKey::default())]
        private_key_or_public_key: PrivateKeyOrPublicKey,
    },
}

#[derive(Clone, Default, clap::ValueEnum)]
enum PrivateKeyOrPublicKey {
    PrivateKey,
    #[default]
    PublicKey,
}

impl PrivateKeyOrPublicKey {
    fn is_private_key(&self) -> bool {
        match self {
            PrivateKeyOrPublicKey::PrivateKey => true,
            PrivateKeyOrPublicKey::PublicKey => false,
        }
    }
}

#[derive(clap::Subcommand)]
enum MetadataCommand {
    /// Get metadata
    Get,
}

#[derive(clap::Subcommand)]
enum TextNoteCommand {
    /// Create a new note
    Create {
        /// The content of a note
        content: String,
        /// The event id to reply to
        #[arg(long, name = "EVENT_ID")]
        reply_to: Option<String>,
    },
    /// Delete the note
    Delete { event_id: String },
    /// Dislike the note
    Dislike { event_id: String },
    /// Like the note
    Like { event_id: String },
    /// List notes
    List {
        #[arg(long, default_value_t = false)]
        me: bool,
    },
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
            KeypairCommand::Get {
                private_key_or_public_key,
            } => handler::keypair::get(private_key_or_public_key.is_private_key()).await,
        },
        Resource::Metadata { command } => match command {
            MetadataCommand::Get => handler::metadata::get().await,
        },
        Resource::TextNote { command } => match command {
            TextNoteCommand::Create { content, reply_to } => {
                handler::text_note::create(content, reply_to).await
            }
            TextNoteCommand::Delete { event_id } => handler::text_note::delete(event_id).await,
            TextNoteCommand::Dislike { event_id } => handler::text_note::dislike(event_id).await,
            TextNoteCommand::Like { event_id } => handler::text_note::like(event_id).await,
            TextNoteCommand::List { me } => handler::text_note::list(me).await,
        },
    }
}
