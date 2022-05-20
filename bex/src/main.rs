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
    println!("{:#?}", credential);

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

    println!("{:#?}", response_body);

    #[derive(Debug)]
    struct RetrieveResponse {
        item_id: String,
        // resolved_id - A unique identifier similar to the item_id but is unique to the actual url of the saved item. The resolved_id identifies unique urls. For example a direct link to a New York Times article and a link that redirects (ex a shortened bit.ly url) to the same article will share the same resolved_id. If this value is 0, it means that Pocket has not processed the item. Normally this happens within seconds but is possible you may request the item before it has been resolved.
        // given_url - The actual url that was saved with the item. This url should be used if the user wants to view the item.
        // resolved_url - The final url of the item. For example if the item was a shortened bit.ly link, this will be the actual article the url linked to.
        // given_title - The title that was saved along with the item.
        // resolved_title - The title that Pocket found for the item when it was parsed
        // favorite - 0 or 1 - 1 If the item is favorited
        // status - 0, 1, 2 - 1 if the item is archived - 2 if the item should be deleted
        // excerpt - The first few lines of the item (articles only)
        // is_article - 0 or 1 - 1 if the item is an article
        // has_image - 0, 1, or 2 - 1 if the item has images in it - 2 if the item is an image
        // has_video - 0, 1, or 2 - 1 if the item has videos in it - 2 if the item is a video
        // word_count - How many words are in the article
        // tags - A JSON object of the user tags associated with the item
        // authors - A JSON object listing all of the authors associated with the item
        // images - A JSON object listing all of the images associated with the item
        // videos - A JSON object listing all of the videos associated with the item
    }

    let list = response_body
        .get("list")
        .map(|v| v.as_object())
        .unwrap()
        .unwrap();
    let (item_id_q, item) = list.iter().next().unwrap();
    let item_id = item
        .as_object()
        .and_then(|o| o.get("item_id"))
        .and_then(|v| v.as_str())
        .unwrap()
        .to_owned();
    if item_id.as_str() != item_id_q.as_str() {
        // TODO: Error
        println!(
            "item_id does not match: item_id = {}, key = {}",
            item_id, item_id_q
        );
    }
    let response = RetrieveResponse { item_id };
    println!("{:#?}", response);

    Ok(())
}
