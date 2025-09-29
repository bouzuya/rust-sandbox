#[derive(Clone)]
pub struct InMemoryEventStore(
    pub(super)  std::sync::Arc<
        std::sync::Mutex<std::collections::HashMap<String, Vec<crate::event::UserEvent>>>,
    >,
);

impl InMemoryEventStore {
    pub fn new() -> Self {
        Self(std::sync::Arc::new(std::sync::Mutex::new(
            std::collections::HashMap::new(),
        )))
    }
}
