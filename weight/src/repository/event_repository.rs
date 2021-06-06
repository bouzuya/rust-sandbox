use crate::set::Set;
use async_trait::async_trait;

#[async_trait]
pub trait EventRepository {
    async fn find_all(&self) -> anyhow::Result<Vec<Set>>;
    async fn save(&self, events: &Vec<Set>) -> anyhow::Result<()>;
}
