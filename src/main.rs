use anyhow::Result;
use tracing_subscriber::{EnvFilter, fmt};

mod analytics;
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

    let listings = match scraper.scrape_category("mobile-phones").await {
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
            listings
        }
        Err(e) => {
            tracing::error!("Failed to scrape: {}", e);
            vec![]
        }
    };

    if !listings.is_empty() {
        use crate::analytics::{PriceEngine, RecommendationEngine};
        use rust_decimal::Decimal;
        use rust_decimal::prelude::FromPrimitive;

        // Extract all prices from listings
        let prices: Vec<Decimal> = listings.iter().filter_map(|l| l.price).collect();

        tracing::info!("Analyzing {} competitor prices", prices.len());

        // Calculate market statistics
        if let Some(stats) = PriceEngine::calculate_statistics_no_outliers(&prices) {
            tracing::info!("Market Analysis:");
            tracing::info!("  Average Price: GHS {}", stats.mean);
            tracing::info!("  Median Price: GHS {}", stats.median);
            tracing::info!("  Price Range: GHS {} - GHS {}", stats.min, stats.max);
            tracing::info!("  25th Percentile: GHS {}", stats.percentile_25);
            tracing::info!("  75th Percentile: GHS {}", stats.percentile_75);
            tracing::info!("  Competitors Analyzed: {}", stats.count);
            if stats.outliers_removed > 0 {
                tracing::info!("  Outliers Removed: {}", stats.outliers_removed);
            }

            // Example recommendation for a hypothetical product
            let example_price = Decimal::from_u64(5000).unwrap();
            let comparison = PriceEngine::compare_to_market(example_price, &stats);
            tracing::info!("");
            tracing::info!(
                "Example Recommendation (Product priced at GHS {}):",
                example_price
            );
            tracing::info!("  {}", comparison.explanation());
        }
    }

    tracing::info!("Spider shutdown complete");
    Ok(())
}
