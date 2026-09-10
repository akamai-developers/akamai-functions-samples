use spin_sdk::variables;

pub(crate) struct Config {
    pub ttl_in_minutes: u64,
    pub use_key_value_store: bool,
    pub upstream_url: String,
}

impl Config {
    pub async fn load() -> anyhow::Result<Self> {
        let url = variables::get("upstream_url").await?;
        let use_kv = variables::get("use_kv_store")
            .await
            .map(|v| v == "true")
            .unwrap_or_default();
        let ttl = variables::get("ttl_in_minutes")
            .await
            .map(|v| v.parse::<u64>().unwrap())?;

        Ok(Self {
            ttl_in_minutes: ttl,
            upstream_url: url,
            use_key_value_store: use_kv,
        })
    }
}
