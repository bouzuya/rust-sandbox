mod fs;

use async_trait::async_trait;

#[async_trait]
pub trait Storage {
    type Key;
    type Value;

    async fn get_item(&self, key: Self::Key) -> anyhow::Result<Option<Self::Value>>;
    async fn keys(&self) -> anyhow::Result<Vec<Self::Key>>;
    async fn remove_item(&self, key: Self::Key) -> anyhow::Result<()>;
    async fn set_item(&self, key: Self::Key, value: Self::Value) -> anyhow::Result<()>;
}
