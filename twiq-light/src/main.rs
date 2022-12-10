use store::{TweetQueueStore, TweetStore};

mod dequeue;
mod domain;
mod enqueue;
mod fetch;
mod google;
mod import;
mod search;
mod store;

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, clap::Subcommand)]
enum Subcommand {
    Dequeue,
    Enqueue { tweet: String },
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
    match args.subcommand {
        Subcommand::Dequeue => dequeue::run(queue_store).await,
        Subcommand::Enqueue { tweet } => enqueue::run(queue_store, tweet).await,
        Subcommand::Fetch => fetch::run(store).await,
        Subcommand::Import { file } => import::run(store, file).await,
        Subcommand::Search { query } => search::run(store, query).await,
    }
}
