mod biscuit;
mod credential_store;

use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use anyhow::Context;
use biscuit::Biscuit;
use clap::{Parser, Subcommand};
use credential_store::Credential;
use pocket::{
    access_token_request, authorization_request, retrieve_request, AccessTokenRequest,
    AuthorizationRequest, RetrieveRequest, RetrieveRequestDetailType, RetrieveRequestState,
};
use serde::{Deserialize, Serialize};
use xdg::BaseDirectories;

use crate::credential_store::CredentialStore;

fn config_dir() -> anyhow::Result<PathBuf> {
    let prefix = "net.bouzuya.rust-sandbox.bex";
    Ok(match env::var_os("BEX_CONFIG_DIR") {
        Some(config_dir) => PathBuf::from(config_dir),
        None => BaseDirectories::with_prefix(prefix)?.get_config_home(),
    })
}

fn config_file<P: AsRef<Path>>(dir: P) -> PathBuf {
    dir.as_ref().join("config.json")
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    consumer_key: String,
}

fn load_config_file<P: AsRef<Path>>(config_file: P) -> anyhow::Result<Option<Config>> {
    let p = config_file.as_ref();
    if p.exists() {
        let s = fs::read_to_string(p)?;
        Ok(serde_json::from_str(&s)?)
    } else {
        Ok(None)
    }
}

fn store_config_file<P: AsRef<Path>>(config_file: P, config: &Config) -> anyhow::Result<()> {
    let p = config_file.as_ref();
    if let Some(dir) = p.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(p, serde_json::to_string_pretty(config)?)?;
    Ok(())
}

fn state_dir() -> anyhow::Result<PathBuf> {
    let prefix = "net.bouzuya.rust-sandbox.bex";
    Ok(match env::var_os("BEX_STATE_DIR") {
        Some(state_dir) => PathBuf::from(state_dir),
        None => BaseDirectories::with_prefix(prefix)?.get_state_home(),
    })
}

async fn authorize(consumer_key: &str) -> anyhow::Result<Credential> {
    // Step 1: Obtain a platform consumer key
    let redirect_uri = "pocketapp1234:authorizationFinished";
    let state = "state1";

    // Step 2: Obtain a request token
    let response_body = authorization_request(&AuthorizationRequest {
        consumer_key,
        redirect_uri,
        state: Some(state),
    })
    .await?;
    let request_token = response_body.code;

    // Step 3: Redirect user to Pocket to continue authorization
    // TODO: encode uri component
    let redirect_url = format!(
        "https://getpocket.com/auth/authorize?request_token={}&redirect_uri={}",
        request_token, redirect_uri
    );
    println!("{}", redirect_url);

    // Step 4: Receive the callback from Pocket
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    // Step 5: Convert a request token into a Pocket access token
    let response_body = access_token_request(&AccessTokenRequest {
        consumer_key,
        code: request_token.as_str(),
    })
    .await?;
    if response_body.state.as_deref() != Some(state) {
        // TODO: Error
        println!(
            "state does not match: expected {}, actual {:?}",
            state,
            response_body.state.as_deref()
        );
    }

    let credential = Credential::new(response_body.access_token, response_body.username);

    Ok(credential)
}

#[derive(Debug, Parser)]
#[clap(version)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Delete,
    List {
        #[clap(long)]
        consumer_key: Option<String>,
    },
    Login {
        #[clap(long)]
        consumer_key: Option<String>,
    },
    Logout,
    Open,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();

    match args.command {
        Commands::Delete => todo!(),
        Commands::List { consumer_key } => list(consumer_key).await?,
        Commands::Login { consumer_key } => login(consumer_key).await?,
        Commands::Logout => logout().await?,
        Commands::Open => todo!(),
    }
    Ok(())
}

async fn list(consumer_key: Option<String>) -> anyhow::Result<()> {
    let consumer_key = consumer_key
        .or_else(|| env::var("CONSUMER_KEY").ok())
        .context("consumer_key is not specified")?;
    let state_dir = state_dir()?;
    let credential_store = CredentialStore::new(state_dir.as_path());
    let credential = credential_store.load()?.context("Not logged in")?;
    let access_token = credential.access_token;
    let response_body = retrieve_request(&RetrieveRequest {
        consumer_key: consumer_key.as_str(),
        access_token: access_token.as_str(),
        state: Some(RetrieveRequestState::Unread),
        favorite: None,
        tag: None,
        content_type: None,
        sort: None,
        detail_type: Some(RetrieveRequestDetailType::Simple),
        search: None,
        domain: None,
        since: None,
        count: Some(3),
        offset: None,
    })
    .await?;
    // println!("{:#?}", response_body);

    let mut biscuits = response_body
        .list
        .into_iter()
        .map(|(_, item)| Biscuit::try_from(item))
        .collect::<anyhow::Result<Vec<Biscuit>>>()?;
    biscuits.sort();
    serde_json::to_writer(io::stdout(), &biscuits)?;
    Ok(())
}

async fn login(consumer_key: Option<String>) -> anyhow::Result<()> {
    let config_dir = config_dir()?;
    let config_file = config_file(config_dir);
    let config_consumer_key = load_config_file(config_file)?.map(|config| config.consumer_key);
    let consumer_key = consumer_key
        .or_else(|| env::var("CONSUMER_KEY").ok())
        .or(config_consumer_key)
        .context("consumer_key is not specified")?;

    let state_dir = state_dir()?;
    let credential_store = CredentialStore::new(state_dir.as_path());
    match credential_store.load()? {
        Some(_) => {
            // do nothing
        }
        None => {
            let credential = authorize(consumer_key.as_str()).await?;
            credential_store.store(&credential)?
        }
    }
    println!("Logged in");
    Ok(())
}

async fn logout() -> anyhow::Result<()> {
    let state_dir = state_dir()?;
    let credential_store = CredentialStore::new(state_dir.as_path());
    credential_store.delete()?;
    println!("Logged out");
    Ok(())
}
