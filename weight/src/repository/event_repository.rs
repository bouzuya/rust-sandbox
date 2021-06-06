use crate::event::Event;
use async_trait::async_trait;

#[async_trait]
pub trait EventRepository {
    async fn find_all(&self) -> anyhow::Result<Vec<Event>>;
    async fn save(&self, events: &Vec<Event>) -> anyhow::Result<()>;
}
