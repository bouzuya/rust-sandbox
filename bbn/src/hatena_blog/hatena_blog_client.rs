use crate::hatena_blog::HatenaBlogEntryId;
use hatena_blog::{
    Client, Config, CreateEntryResponse, EntryId, EntryParams, GetEntryResponse,
    ListEntriesResponse, UpdateEntryResponse,
};

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
    ) -> anyhow::Result<GetEntryResponse> {
        let client = Client::new(&self.config);
        let entry_id = EntryId::from(hatena_blog_entry_id);
        Ok(client.get_entry(&entry_id).await?)
    }

    pub async fn list_entries_in_page(
        &self,
        page: Option<&str>,
    ) -> anyhow::Result<ListEntriesResponse> {
        let client = Client::new(&self.config);
        Ok(client.list_entries_in_page(page).await?)
    }
}
