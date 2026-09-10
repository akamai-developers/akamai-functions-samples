use anyhow::Context;
use serde::{Deserialize, Serialize};
use spin_sdk::key_value::Store;

#[derive(Serialize, Deserialize)]
pub(crate) struct CacheItem {
    pub timestamp: u64,
    pub value: String,
}

impl CacheItem {
    pub(crate) fn is_valid(&self, ttl_in_minutes: u64) -> bool {
        self.timestamp + ttl_in_minutes * 60
            > std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
    }

    pub(crate) async fn load_by_key(kv: &Store, key: &str) -> anyhow::Result<Option<Self>> {
        kv.get_json::<Self>(key).await
    }

    pub(crate) async fn store_at_key(&self, kv: &Store, key: &str) -> anyhow::Result<()> {
        kv.set_json(key, self).await
    }

    pub(crate) async fn delete_by_key(kv: &Store, key: &str) -> anyhow::Result<()> {
        kv.delete(key)
            .await
            .context("Error while deleting CacheItem from kv store")
    }

    pub(crate) fn new(value: String) -> Self {
        Self {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            value,
        }
    }
}
