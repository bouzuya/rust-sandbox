pub struct InMemoryUserReader {
    store: std::sync::Arc<std::sync::Mutex<Vec<crate::query_models::QueryUser>>>,
}

impl InMemoryUserReader {
    pub fn new() -> Self {
        Self {
            store: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl crate::readers::UserReader for InMemoryUserReader {
    async fn list(
        &self,
    ) -> Result<Vec<crate::query_models::QueryUser>, crate::readers::UserReaderError> {
        let store = self.store.lock().unwrap();
        Ok(store.to_vec())
    }
}
