use crate::config::Config;
use anyhow::Result;
use reqwest::Client;

pub struct PhoenixClient {
    _client: Client,
    _base_url: String,
    _token: String,
}

impl PhoenixClient {
    pub fn new(config: &Config) -> Self {
        Self {
            _client: Client::new(),
            _base_url: config.phoenix_api_url.clone(),
            _token: config.phoenix_api_token.clone(),
        }
    }

    // Placeholder for future implementation
    pub async fn health_check(&self) -> Result<()> {
        tracing::debug!("Phoenix client initialized (placeholder)");
        Ok(())
    }
}
