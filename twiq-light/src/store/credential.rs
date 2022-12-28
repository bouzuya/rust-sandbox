use crate::{
    credential::Credential,
    storage::{firestore::FirestoreStorage, Storage},
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
