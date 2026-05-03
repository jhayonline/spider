#![allow(clippy::all)]
use crate::config::Config;
use crate::models::CompetitorListing;
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct BatchPriceIntel {
    listings: Vec<CompetitorListing>,
    analysis: Vec<serde_json::Value>,
}

pub struct PhoenixClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl PhoenixClient {
    pub fn new(config: &Config) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .expect("Failed to build HTTP client"),
            base_url: config.phoenix_api_url.clone(),
            api_key: config.phoenix_api_token.clone(),
        }
    }

    pub async fn send_batch_intel(&self, listings: Vec<CompetitorListing>) -> Result<()> {
        let total_listings = listings.len();
        let batch_size = 500; // Send 500 listings at a time
        let total_batches = (total_listings + batch_size - 1) / batch_size;

        if total_batches == 1 {
            // Send all at once for small batches
            let url = format!("{}/api/price-intel/batch", self.base_url);
            let batch = BatchPriceIntel {
                listings,
                analysis: vec![],
            };

            let response = self
                .client
                .post(&url)
                .header("X-API-Key", &self.api_key)
                .json(&batch)
                .send()
                .await?;

            if response.status().is_success() {
                tracing::info!("All {} listings sent in one batch", total_listings);
                Ok(())
            } else {
                let error = response.text().await?;
                anyhow::bail!("Failed to send: {}", error)
            }
        } else {
            tracing::info!(
                "Sending {} listings in {} batches",
                total_listings,
                total_batches
            );

            for (i, chunk) in listings.chunks(batch_size).enumerate() {
                let url = format!("{}/api/price-intel/batch", self.base_url);
                let batch = BatchPriceIntel {
                    listings: chunk.to_vec(),
                    analysis: vec![],
                };

                let response = self
                    .client
                    .post(&url)
                    .header("X-API-Key", &self.api_key)
                    .json(&batch)
                    .send()
                    .await?;

                if response.status().is_success() {
                    tracing::debug!("Batch {}/{} sent", i + 1, total_batches);
                } else {
                    let error = response.text().await?;
                    anyhow::bail!("Failed to send batch {}: {}", i + 1, error);
                }
            }

            tracing::info!("All {} batches sent successfully", total_batches);
            Ok(())
        }
    }
}
