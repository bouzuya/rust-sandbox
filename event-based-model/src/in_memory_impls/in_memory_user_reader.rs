use super::InMemoryReadModelStore;

pub struct InMemoryUserReader {
    read_model_store: InMemoryReadModelStore,
}

impl InMemoryUserReader {
    pub fn new(read_model_store: InMemoryReadModelStore) -> Self {
        Self { read_model_store }
    }
}

#[async_trait::async_trait]
impl crate::readers::UserReader for InMemoryUserReader {
    async fn list(
        &self,
    ) -> Result<Vec<crate::query_models::QueryUser>, crate::readers::UserReaderError> {
        let store = self.read_model_store.0.lock().unwrap();
        Ok(store.to_vec())
    }
}
