use std::{env, io};

use crate::request::{
    access_token_request, authorization_request, retrieve_request, AccessTokenRequest,
    AuthorizationRequest, RetrieveRequest, RetrieveRequestDetailType, RetrieveRequestState,
};

mod request;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Step 1: Obtain a platform consumer key
    let consumer_key = env::var("CONSUMER_KEY")?;
    let redirect_uri = "pocketapp1234:authorizationFinished";
    let state = "state1";

    // Step 2: Obtain a request token
    let response_body = authorization_request(&AuthorizationRequest {
        consumer_key: consumer_key.as_str(),
        redirect_uri,
        state: Some(state),
    })
    .await?;
    println!("{:#?}", response_body);
    let request_token = response_body.code;
    println!("request_token = {}", request_token);

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
        consumer_key: consumer_key.as_str(),
        code: request_token.as_str(),
    })
    .await?;
    println!("{:#?}", response_body);
    if response_body.state.as_deref() != Some(state) {
        // TODO: Error
        println!(
            "state does not match: expected {}, actual {:?}",
            state,
            response_body.state.as_deref()
        );
    }
    let access_token = response_body.access_token;
    println!("access_token = {}", access_token);
    let username = response_body.username;
    println!("username = {}", username);

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
        count: Some(3),
    })
    .await?;
    println!("{:#?}", response_body);

    Ok(())
}
