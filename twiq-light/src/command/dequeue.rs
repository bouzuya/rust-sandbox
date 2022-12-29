use anyhow::bail;
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};
use tracing::{debug, instrument};

use crate::{
    store::TweetQueueStore,
    token::Token,
    twitter::{self, PostTweetsRequestBody},
};

async fn ensure_token(
    store: &TweetQueueStore,
    client_id: &str,
    client_secret: &str,
) -> anyhow::Result<Token> {
    let token = store.read_token().await?;
    match token {
        Some(token) => {
            let expires = OffsetDateTime::parse(&token.expires, &Rfc3339)?;
            if OffsetDateTime::now_utc() < expires - Duration::seconds(10) {
                Ok(token)
            } else {
                // use refresh token
                let access_token_response = twitter::refresh_access_token(
                    client_id,
                    client_secret,
                    token.refresh_token.as_str(),
                )
                .await?;
                debug!("{:?}", access_token_response);

                let token = Token::try_from(
                    access_token_response,
                    OffsetDateTime::now_utc().unix_timestamp(),
                )?;

                store.write_token(&token).await?;

                Ok(token)
            }
        }
        None => bail!("Use `twiq-light queue authorize`"),
    }
}

#[instrument(skip_all)]
pub async fn run(
    store: TweetQueueStore,
    client_id: String,
    client_secret: String,
) -> anyhow::Result<()> {
    let token = ensure_token(&store, &client_id, &client_secret).await?;
    debug!("{:?}", token);
    let mut queue = store.read_all().await?;
    if let Some(item) = queue.pop_front() {
        twitter::post_tweets(
            &token.access_token,
            PostTweetsRequestBody {
                text: Some(item.text),
            },
        )
        .await?;
        store.write_all(&queue).await?;
    }
    Ok(())
}
