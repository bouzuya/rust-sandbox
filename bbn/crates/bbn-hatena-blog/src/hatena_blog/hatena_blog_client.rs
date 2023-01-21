use crate::hatena_blog::HatenaBlogEntryId;
use hatena_blog_api::{
    Client, Config, CreateEntryResponse, EntryId, EntryParams, GetEntryResponse,
    UpdateEntryResponse,
};

use crate::hatena_blog::HatenaBlogListEntriesResponse;

#[derive(Clone, Debug)]
pub struct HatenaBlogClient {
    config: Config,
}

impl HatenaBlogClient {
    pub fn new(hatena_id: String, hatena_blog_id: String, hatena_api_key: String) -> Self {
        let config = Config::new(&hatena_id, None, &hatena_blog_id, &hatena_api_key);
        Self { config }
    }

    pub async fn create_entry(&self, params: EntryParams) -> anyhow::Result<CreateEntryResponse> {
        let client = Client::new(&self.config);
        Ok(client.create_entry(params).await?)
    }

    pub async fn update_entry(
        &self,
        hatena_blog_entry_id: &HatenaBlogEntryId,
        params: EntryParams,
    ) -> anyhow::Result<UpdateEntryResponse> {
        let client = Client::new(&self.config);
        let entry_id = EntryId::from(hatena_blog_entry_id);
        Ok(client.update_entry(&entry_id, params).await?)
    }

    pub async fn get_entry(
        &self,
        hatena_blog_entry_id: &HatenaBlogEntryId,
    ) -> anyhow::Result<Option<GetEntryResponse>> {
        let client = Client::new(&self.config);
        let entry_id = EntryId::from(hatena_blog_entry_id);
        Ok(match client.get_entry(&entry_id).await {
            Ok(response) => Ok(Some(response)),
            Err(err) => match err {
                hatena_blog_api::ClientError::NotFound => Ok(None),
                hatena_blog_api::ClientError::RequestError(_)
                | hatena_blog_api::ClientError::BadRequest
                | hatena_blog_api::ClientError::Unauthorized
                | hatena_blog_api::ClientError::MethodNotAllowed
                | hatena_blog_api::ClientError::InternalServerError
                | hatena_blog_api::ClientError::UnknownStatusCode => Err(err),
            },
        }?)
    }

    pub async fn list_entries_in_page(
        &self,
        page: Option<&str>,
    ) -> anyhow::Result<HatenaBlogListEntriesResponse> {
        let client = Client::new(&self.config);
        Ok(HatenaBlogListEntriesResponse::from(
            client.list_entries_in_page(page).await?,
        ))
    }
}
