mod authorize;
mod dequeue;
mod domain;
mod enqueue;
mod fetch;
mod google;
mod import;
mod list_queue;
mod remove;
mod reorder;
mod search;
mod storage;
mod store;
mod token;
mod tweet_store;
mod twitter;

use anyhow::Context;
use store::TweetQueueStore;
use tweet_store::TweetStore;

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

#[derive(Clone, Debug, clap::Args)]
struct ConfigOptions {
    #[arg(long, env = "TWIQ_LIGHT_GOOGLE_PROJECT_ID")]
    project_id: Option<String>,
    #[arg(long, env = "TWIQ_LIGHT_GOOGLE_APPLICATION_CREDENTIALS")]
    google_application_credentials: Option<String>,
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
        #[command(flatten)]
        config: ConfigOptions,
    },
    Dequeue {
        #[arg(long, env = "TWIQ_LIGHT_TWITTER_CLIENT_ID")]
        client_id: String,
        #[arg(long, env = "TWIQ_LIGHT_TWITTER_CLIENT_SECRET")]
        client_secret: String,
        #[command(flatten)]
        config: ConfigOptions,
    },
    Enqueue {
        tweet: String,
        #[command(flatten)]
        config: ConfigOptions,
    },
    List {
        #[command(flatten)]
        config: ConfigOptions,
    },
    Remove {
        index: usize,
        #[command(flatten)]
        config: ConfigOptions,
    },
    Reorder {
        src: usize,
        dst: usize,
        #[command(flatten)]
        config: ConfigOptions,
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

fn tweet_queue_store(config: ConfigOptions) -> anyhow::Result<TweetQueueStore> {
    Ok(TweetQueueStore::new(
        config.project_id.context("project_id")?,
        config.google_application_credentials,
    ))
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
                config,
            } => {
                authorize::run(
                    tweet_queue_store(config)?,
                    client_id,
                    client_secret,
                    redirect_uri,
                )
                .await
            }
            QueueSubcommand::Dequeue {
                client_id,
                client_secret,
                config,
            } => dequeue::run(tweet_queue_store(config)?, client_id, client_secret).await,
            QueueSubcommand::Enqueue { tweet, config } => {
                enqueue::run(tweet_queue_store(config)?, tweet).await
            }
            QueueSubcommand::List { config } => list_queue::run(tweet_queue_store(config)?).await,
            QueueSubcommand::Remove { index, config } => {
                remove::run(tweet_queue_store(config)?, index).await
            }
            QueueSubcommand::Reorder { src, dst, config } => {
                reorder::run(tweet_queue_store(config)?, src, dst).await
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
