use anyhow::Result;
use tracing_subscriber::{EnvFilter, fmt};

mod clients;
mod config;
mod models;

use clients::jiji::JijiScraper;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging with env filter
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("spider=debug"));

    fmt().with_env_filter(filter).init();

    tracing::info!("Starting Spider Intelligence Engine");

    // Load configuration
    let config = config::Config::from_env()?;
    tracing::debug!("Configuration loaded: {:?}", config);

    // Test Jiji scraping
    let scraper = JijiScraper::new(&config.user_agent);

    match scraper.scrape_category("mobile-phones").await {
        Ok(listings) => {
            let listings: Vec<crate::models::ScrapedListing> = listings;
            tracing::info!("Successfully scraped {} listings", listings.len());
            for listing in listings.iter().take(5) {
                tracing::debug!(
                    "Product: {} | Price: {:?} | URL: {}",
                    listing.title,
                    listing.price,
                    listing.url
                );
            }
        }
        Err(e) => {
            tracing::error!("Failed to scrape: {}", e);
        }
    }

    tracing::info!("Spider shutdown complete");
    Ok(())
}
