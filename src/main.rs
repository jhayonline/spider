use anyhow::Result;
use tracing_subscriber::{EnvFilter, fmt};

mod analytics;
mod clients;
mod config;
mod models;

use clients::jiji::JijiScraper;
use clients::phoenix::PhoenixClient;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("spider=info"));
    fmt().with_env_filter(filter).init();

    tracing::info!("Starting Spider Intelligence Engine");

    // Load configuration
    let config = config::Config::from_env()?;
    tracing::debug!("Configuration loaded");

    // Initialize Phoenix client
    let phoenix = PhoenixClient::new(&config);

    // Scrape Jiji with pagination (3 pages = ~72 listings)
    let scraper = JijiScraper::new(&config.user_agent);
    let max_pages = 3;

    tracing::info!("Scraping up to {} pages from Jiji", max_pages);
    let listings = scraper
        .scrape_category_paginated("mobile-phones", max_pages)
        .await?;

    tracing::info!("Scraped {} total listings", listings.len());

    // Prepare competitor listings for API
    let competitor_listings: Vec<models::CompetitorListing> = listings
        .iter()
        .filter_map(|l| {
            l.price.map(|price| models::CompetitorListing {
                product_title: l.title.clone(),
                price,
                condition: l.condition.clone(),
                platform: "Jiji".to_string(),
                location: l.location.clone(),
                url: l.url.clone(),
            })
        })
        .collect();

    tracing::info!(
        "Prepared {} competitor listings with prices",
        competitor_listings.len()
    );

    // Analyze prices
    let prices: Vec<rust_decimal::Decimal> = listings.iter().filter_map(|l| l.price).collect();

    if !prices.is_empty() {
        use analytics::PriceEngine;

        if let Some(stats) = PriceEngine::calculate_statistics_no_outliers(&prices) {
            tracing::info!("Market Analysis (based on {} listings):", stats.count);
            tracing::info!("  Average Price: GHS {}", stats.mean.round_dp(2));
            tracing::info!("  Median Price: GHS {}", stats.median);
            tracing::info!("  Price Range: GHS {} - GHS {}", stats.min, stats.max);
            if stats.outliers_removed > 0 {
                tracing::info!("  Outliers Removed: {}", stats.outliers_removed);
            }

            // Send to Phoenix Mall
            match phoenix.send_batch_intel(competitor_listings).await {
                Ok(_) => tracing::info!("Data sent to Phoenix Mall successfully"),
                Err(e) => tracing::error!("Failed to send to Phoenix Mall: {}", e),
            }
        }
    } else {
        tracing::warn!("No valid prices found in scraped listings");
    }

    tracing::info!("Spider shutdown complete");
    Ok(())
}
