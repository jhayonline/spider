use crate::config::Config;
use crate::models::{CompetitorListing, PriceIntelData};
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct BatchPriceIntel {
    listings: Vec<CompetitorListing>,
    analysis: Vec<PriceIntelData>,
}

pub struct PhoenixClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl PhoenixClient {
    pub fn new(config: &Config) -> Self {
        Self {
            client: Client::new(),
            base_url: config.phoenix_api_url.clone(),
            api_key: config.phoenix_api_token.clone(),
        }
    }

    pub async fn send_batch_intel(
        &self,
        listings: Vec<CompetitorListing>,
        analysis: Vec<PriceIntelData>,
    ) -> Result<()> {
        let url = format!("{}/api/price-intel/batch", self.base_url);

        let batch = BatchPriceIntel { listings, analysis };

        let response = self
            .client
            .post(&url)
            .header("X-API-Key", &self.api_key) // Use API key header instead of Bearer
            .json(&batch)
            .send()
            .await?;

        if response.status().is_success() {
            tracing::info!("Successfully sent price intel to Phoenix Mall");
            Ok(())
        } else {
            let error = response.text().await?;
            anyhow::bail!("Failed to send price intel: {}", error)
        }
    }
}
