use anyhow::Context;
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};
use tracing::debug;

use crate::{
    credential::Credential,
    storage::{firestore::FirestoreStorage, Storage},
    token::Token,
    twitter,
};

pub struct CredentialStore {
    storage: FirestoreStorage,
}

impl CredentialStore {
    const DATABASE_ID: &str = "(default)";
    const COLLECTION_ID: &str = "twiq-light";
    const DOCUMENT_ID: &str = "credential";

    pub async fn new(
        project_id: String,
        google_application_credentials: Option<String>,
    ) -> anyhow::Result<Self> {
        let storage = FirestoreStorage::new(
            google_application_credentials,
            project_id,
            Self::DATABASE_ID.to_owned(),
            Self::COLLECTION_ID.to_owned(),
        )
        .await?;
        Ok(Self { storage })
    }

    pub async fn ensure_token(&self) -> anyhow::Result<Token> {
        let stored = self
            .read()
            .await?
            .context("Use `twiq-light queue authorize`")?;

        let expires = OffsetDateTime::parse(&stored.token.expires, &Rfc3339)?;
        if OffsetDateTime::now_utc() < expires - Duration::seconds(10) {
            Ok(stored.token)
        } else {
            // use refresh token
            let access_token_response = twitter::refresh_access_token(
                &stored.client.id,
                &stored.client.secret,
                stored.token.refresh_token.as_str(),
            )
            .await?;
            debug!("{:?}", access_token_response);

            let token = Token::try_from(
                access_token_response,
                OffsetDateTime::now_utc().unix_timestamp(),
            )?;

            let refreshed = Credential {
                client: stored.client,
                token,
            };
            self.write(&refreshed).await?;

            Ok(refreshed.token)
        }
    }

    pub async fn read(&self) -> anyhow::Result<Option<Credential>> {
        self.storage
            .get_item(Self::DOCUMENT_ID.to_owned())
            .await?
            .map(|s| Ok(serde_json::from_str::<'_, Credential>(s.as_str())?))
            .transpose()
    }

    pub async fn write(&self, credential: &Credential) -> anyhow::Result<()> {
        self.storage
            .set_item(
                Self::DOCUMENT_ID.to_owned(),
                serde_json::to_string(&credential)?,
            )
            .await
    }
}
