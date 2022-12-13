use store::{TweetQueueStore, TweetStore};

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
    Dequeue,
    Enqueue { tweet: String },
    List,
    Reorder { src: usize, dst: usize },
}

#[derive(Clone, Debug, clap::Subcommand)]
enum TweetSubcommand {
    Fetch,
    Import { file: String },
    Search { query: Option<String> },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let queue_store = TweetQueueStore::default();
    let store = TweetStore::default();
    let args = <Args as clap::Parser>::parse();
    match args.resource {
        Resource::Queue(command) => match command {
            QueueSubcommand::Dequeue => dequeue::run(queue_store).await,
            QueueSubcommand::Enqueue { tweet } => enqueue::run(queue_store, tweet).await,
            QueueSubcommand::List => list_queue::run(queue_store).await,
            QueueSubcommand::Reorder { src, dst } => reorder::run(queue_store, src, dst).await,
        },
        Resource::Tweet(command) => match command {
            TweetSubcommand::Fetch => fetch::run(store).await,
            TweetSubcommand::Import { file } => import::run(store, file).await,
            TweetSubcommand::Search { query } => search::run(store, query).await,
        },
    }
}
