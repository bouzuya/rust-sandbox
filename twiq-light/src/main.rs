use store::{TweetQueueStore, TweetStore};

mod authorize;
mod dequeue;
mod domain;
mod enqueue;
mod fetch;
mod google;
mod import;
mod list_queue;
mod reorder;
mod search;
mod store;
mod twitter;

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    resource: Resource,
}

#[derive(Clone, Debug, clap::Subcommand)]
enum Resource {
    #[clap(subcommand)]
    Queue(QueueSubcommand),
    #[clap(subcommand)]
    Tweet(TweetSubcommand),
}

#[derive(Clone, Debug, clap::Subcommand)]
enum QueueSubcommand {
    Authorize {
        #[arg(long, env = "TWIQ_LIGHT_TWITTER_CLIENT_ID")]
        client_id: String,
        #[arg(long, env = "TWIQ_LIGHT_TWITTER_CLIENT_SECRET")]
        client_secret: String,
        #[arg(long, env = "TWIQ_LIGHT_TWITTER_REDIRECT_URI")]
        redirect_uri: String,
        #[arg(long, env = "TWIQ_LIGHT_GOOGLE_APPLICATION_CREDENTIALS")]
        google_application_credentials: Option<String>,
    },
    Dequeue {
        #[arg(long, env = "TWIQ_LIGHT_TWITTER_CLIENT_ID")]
        client_id: String,
        #[arg(long, env = "TWIQ_LIGHT_TWITTER_CLIENT_SECRET")]
        client_secret: String,
        #[arg(long, env = "TWIQ_LIGHT_GOOGLE_APPLICATION_CREDENTIALS")]
        google_application_credentials: Option<String>,
    },
    Enqueue {
        tweet: String,
        #[arg(long, env = "TWIQ_LIGHT_GOOGLE_APPLICATION_CREDENTIALS")]
        google_application_credentials: Option<String>,
    },
    List {
        #[arg(long, env = "TWIQ_LIGHT_GOOGLE_APPLICATION_CREDENTIALS")]
        google_application_credentials: Option<String>,
    },
    Reorder {
        src: usize,
        dst: usize,
        #[arg(long, env = "TWIQ_LIGHT_GOOGLE_APPLICATION_CREDENTIALS")]
        google_application_credentials: Option<String>,
    },
}

#[derive(Clone, Debug, clap::Subcommand)]
enum TweetSubcommand {
    Fetch {
        #[arg(long, env = "TWIQ_LIGHT_TWITTER_BEARER_TOKEN")]
        bearer_token: String,
    },
    Import {
        file: String,
    },
    Search {
        query: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = <Args as clap::Parser>::parse();
    match args.resource {
        Resource::Queue(command) => match command {
            QueueSubcommand::Authorize {
                client_id,
                client_secret,
                redirect_uri,
                google_application_credentials,
            } => {
                authorize::run(
                    TweetQueueStore::new(google_application_credentials),
                    client_id,
                    client_secret,
                    redirect_uri,
                )
                .await
            }
            QueueSubcommand::Dequeue {
                client_id,
                client_secret,
                google_application_credentials,
            } => {
                dequeue::run(
                    TweetQueueStore::new(google_application_credentials),
                    client_id,
                    client_secret,
                )
                .await
            }
            QueueSubcommand::Enqueue {
                tweet,
                google_application_credentials,
            } => enqueue::run(TweetQueueStore::new(google_application_credentials), tweet).await,
            QueueSubcommand::List {
                google_application_credentials,
            } => list_queue::run(TweetQueueStore::new(google_application_credentials)).await,
            QueueSubcommand::Reorder {
                src,
                dst,
                google_application_credentials,
            } => {
                reorder::run(
                    TweetQueueStore::new(google_application_credentials),
                    src,
                    dst,
                )
                .await
            }
        },
        Resource::Tweet(command) => {
            let store = TweetStore::default();
            match command {
                TweetSubcommand::Fetch { bearer_token } => fetch::run(store, bearer_token).await,
                TweetSubcommand::Import { file } => import::run(store, file).await,
                TweetSubcommand::Search { query } => search::run(store, query).await,
            }
        }
    }
}
