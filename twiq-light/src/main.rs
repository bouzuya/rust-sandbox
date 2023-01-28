mod command;
mod credential;
mod data;
mod google;
mod storage;
mod store;
mod twitter;

use anyhow::Context;
use data::Config;
use store::{ConfigStore, CredentialStore, TweetQueueStore, TweetStore};

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
        #[command(flatten)]
        config: ConfigOptions,
    },
    Enqueue {
        tweet: String,
        #[arg(long)]
        reply: Option<String>,
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
        #[command(flatten)]
        config: ConfigOptions,
    },
    Import {
        file: String,
    },
    Search {
        query: Option<String>,
    },
}

async fn ensure_config(config_store: ConfigStore, config: ConfigOptions) -> anyhow::Result<Config> {
    Ok(match config_store.read().await? {
        None => Config {
            project_id: config
                .project_id
                .context("no TWIQ_LIGHT_GOOGLE_PROJECT_ID")?,
            google_application_credentials: config.google_application_credentials,
        },
        Some(config) => config,
    })
}

async fn tweet_queue_store(config: Config) -> anyhow::Result<TweetQueueStore> {
    TweetQueueStore::new(config.project_id, config.google_application_credentials).await
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let config_store = ConfigStore::default();
    let args = <Args as clap::Parser>::parse();
    match args.resource {
        Resource::Queue(command) => match command {
            QueueSubcommand::Authorize {
                client_id,
                client_secret,
                redirect_uri,
                config,
            } => {
                command::authorize::run(
                    config_store,
                    config
                        .project_id
                        .context("no TWIQ_LIGHT_GOOGLE_PROJECT_ID")?,
                    config
                        .google_application_credentials
                        .context("no TWIQ_LIGHT_GOOGLE_APPLICATION_CREDENTIALS")?,
                    client_id,
                    client_secret,
                    redirect_uri,
                )
                .await
            }
            QueueSubcommand::Dequeue { config } => {
                let config = ensure_config(config_store, config).await?;
                command::dequeue::run(
                    tweet_queue_store(config.clone()).await?,
                    CredentialStore::new(config.project_id, config.google_application_credentials)
                        .await?,
                )
                .await
            }
            QueueSubcommand::Enqueue {
                reply,
                tweet,
                config,
            } => {
                let config = ensure_config(config_store, config).await?;
                command::enqueue::run(tweet_queue_store(config).await?, tweet, reply).await
            }
            QueueSubcommand::List { config } => {
                let config = ensure_config(config_store, config).await?;
                command::list_queue::run(tweet_queue_store(config).await?).await
            }
            QueueSubcommand::Remove { index, config } => {
                let config = ensure_config(config_store, config).await?;
                command::remove::run(tweet_queue_store(config).await?, index).await
            }
            QueueSubcommand::Reorder { src, dst, config } => {
                let config = ensure_config(config_store, config).await?;
                command::reorder::run(tweet_queue_store(config).await?, src, dst).await
            }
        },
        Resource::Tweet(command) => {
            let store = TweetStore::default();
            match command {
                TweetSubcommand::Fetch { config } => {
                    let config = ensure_config(config_store, config).await?;
                    command::fetch::run(
                        store,
                        CredentialStore::new(
                            config.project_id,
                            config.google_application_credentials,
                        )
                        .await?,
                    )
                    .await
                }
                TweetSubcommand::Import { file } => command::import::run(store, file).await,
                TweetSubcommand::Search { query } => command::search::run(store, query).await,
            }
        }
    }
}
