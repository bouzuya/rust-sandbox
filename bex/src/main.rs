use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use xdg::BaseDirectories;

use crate::request::{
    access_token_request, authorization_request, retrieve_request, AccessTokenRequest,
    AuthorizationRequest, RetrieveRequest, RetrieveRequestDetailType, RetrieveRequestState,
};

mod request;

fn state_dir() -> anyhow::Result<PathBuf> {
    let prefix = "net.bouzuya.rust-sandbox.bex";
    Ok(match env::var_os("BEX_STATE_DIR") {
        Some(state_dir) => PathBuf::from(state_dir),
        None => BaseDirectories::with_prefix(prefix)?.get_state_home(),
    })
}

#[derive(Debug, Deserialize, Serialize)]
struct Credential {
    access_token: String,
    username: String,
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

    let credential = Credential {
        access_token: response_body.access_token,
        username: response_body.username,
    };

    Ok(credential)
}

struct CredentialStore {
    path: PathBuf,
}

impl CredentialStore {
    fn new<P: AsRef<Path>>(dir: P) -> Self {
        Self {
            path: dir.as_ref().join("credential.json"),
        }
    }

    fn load(&self) -> anyhow::Result<Option<Credential>> {
        let p = self.path.as_path();
        if p.exists() {
            let s = fs::read_to_string(p)?;
            Ok(serde_json::from_str(&s)?)
        } else {
            Ok(None)
        }
    }

    fn store(&self, credential: &Credential) -> anyhow::Result<()> {
        let p = self.path.as_path();
        if let Some(dir) = p.parent() {
            fs::create_dir_all(dir)?;
        }
        fs::write(p, serde_json::to_string(credential)?)?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let consumer_key = env::var("CONSUMER_KEY")?;
    let state_dir = state_dir()?;
    let credential_store = CredentialStore::new(state_dir.as_path());
    let credential = match credential_store.load()? {
        Some(c) => c,
        None => {
            let credential = authorize(consumer_key.as_str()).await?;
            credential_store.store(&credential)?;
            credential
        }
    };
    // println!("{:#?}", credential);

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

    #[derive(Debug, Serialize)]
    struct Item {
        title: String,
        url: String,
        added_at: String,
    }

    let items = response_body
        .list
        .into_iter()
        .map(|(_, item)| Item {
            title: item.given_title,
            url: item.given_url,
            added_at: item.time_added.unwrap(),
        })
        .collect::<Vec<Item>>();
    serde_json::to_writer(io::stdout(), &items)?;

    Ok(())
}
