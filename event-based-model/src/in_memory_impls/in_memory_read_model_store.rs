#[derive(Clone)]
pub struct InMemoryReadModelStore(
    pub(super) std::sync::Arc<std::sync::Mutex<Vec<crate::query_models::QueryUser>>>,
);

impl InMemoryReadModelStore {
    pub fn new() -> Self {
        Self(std::sync::Arc::new(std::sync::Mutex::new(Vec::new())))
    }
}
