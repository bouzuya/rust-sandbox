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
    },
    Dequeue {
        #[arg(long, env = "TWIQ_LIGHT_TWITTER_CLIENT_ID")]
        client_id: String,
        #[arg(long, env = "TWIQ_LIGHT_TWITTER_CLIENT_SECRET")]
        client_secret: String,
    },
    Enqueue {
        tweet: String,
    },
    List,
    Reorder {
        src: usize,
        dst: usize,
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
        Resource::Queue(command) => {
            let queue_store = TweetQueueStore::default();
            match command {
                QueueSubcommand::Authorize {
                    client_id,
                    client_secret,
                    redirect_uri,
                } => authorize::run(queue_store, client_id, client_secret, redirect_uri).await,
                QueueSubcommand::Dequeue {
                    client_id,
                    client_secret,
                } => dequeue::run(queue_store, client_id, client_secret).await,
                QueueSubcommand::Enqueue { tweet } => enqueue::run(queue_store, tweet).await,
                QueueSubcommand::List => list_queue::run(queue_store).await,
                QueueSubcommand::Reorder { src, dst } => reorder::run(queue_store, src, dst).await,
            }
        }
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
