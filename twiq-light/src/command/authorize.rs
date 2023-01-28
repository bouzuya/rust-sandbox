use std::io;

use anyhow::{bail, Context};
use rand::{rngs::ThreadRng, RngCore};
use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use tracing::{debug, instrument};
use url::Url;

use crate::{
    data::{Config, Credential, Token, TwitterClientKey},
    store::{ConfigStore, CredentialStore},
    twitter,
};

#[instrument(skip_all)]
pub async fn run(
    config_store: ConfigStore,
    project_id: String,
    google_application_credentials: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
) -> anyhow::Result<()> {
    let credential_store = CredentialStore::new(
        project_id.clone(),
        Some(google_application_credentials.clone()),
    )
    .await?;
    let response_type = "code";

    let mut rng = ThreadRng::default();
    let mut state_buf = vec![0; 96];
    rng.fill_bytes(&mut state_buf);
    let mut code_verifier_buf = vec![0; 96];
    rng.fill_bytes(&mut code_verifier_buf);
    let base64_engine = base64::engine::fast_portable::FastPortable::from(
        &base64::alphabet::URL_SAFE,
        base64::engine::fast_portable::NO_PAD,
    );

    let scope = "tweet.read%20tweet.write%20users.read%20offline.access";
    let state = base64::encode_engine(&state_buf, &base64_engine);
    let code_verifier = base64::encode_engine(&code_verifier_buf, &base64_engine);
    let code_challenge =
        base64::encode_engine(Sha256::digest(code_verifier.as_bytes()), &base64_engine);
    let code_challenge_method = "s256";
    println!("https://twitter.com/i/oauth2/authorize?response_type={response_type}&client_id={client_id}&redirect_uri={redirect_uri}&scope={scope}&state={state}&code_challenge={code_challenge}&code_challenge_method={code_challenge_method}");

    // read redirect_uri
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    // parse url and check state
    let url = Url::parse(buffer.trim())?;
    let (_, response_state) = url
        .query_pairs()
        .find(|(k, _)| k == "state")
        .context("state not found")?;
    if response_state.to_string().as_str() != state {
        bail!("state does not match");
    }
    let (_, code) = url
        .query_pairs()
        .find(|(k, _)| k == "code")
        .context("code not found")?;
    let code = code.to_string();

    let access_token_response = twitter::issue_token(
        &client_id,
        &client_secret,
        code.as_str(),
        &redirect_uri,
        &code_verifier,
    )
    .await?;

    debug!("{:?}", access_token_response);

    let token = Token::try_from(
        access_token_response,
        OffsetDateTime::now_utc().unix_timestamp(),
    )?;

    let credential = Credential {
        client: TwitterClientKey {
            id: client_id,
            secret: client_secret,
        },
        token,
    };

    credential_store.write(&credential).await?;
    config_store
        .write(&Config {
            project_id,
            google_application_credentials: Some(google_application_credentials),
        })
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn base64_test() {
        let mut rng = ThreadRng::default();
        let mut state_buf = vec![0; 96];
        rng.fill_bytes(&mut state_buf);
        let base64_engine = base64::engine::fast_portable::FastPortable::from(
            &base64::alphabet::URL_SAFE,
            base64::engine::fast_portable::NO_PAD,
        );
        let state = base64::encode_engine(&state_buf, &base64_engine);
        assert_eq!(state.len(), 128);
    }
}
