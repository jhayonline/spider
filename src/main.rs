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
    tracing::info!(
        "Loaded {} categories to scrape",
        config.scrape_categories.len()
    );

    // Initialize Phoenix client
    let phoenix = PhoenixClient::new(&config);

    // Scrape multiple categories
    let scraper = JijiScraper::new(&config.user_agent);
    let max_pages = config.max_pages_per_category;

    let mut all_competitor_listings = Vec::new();
    let mut total_listings = 0;

    for category in &config.scrape_categories {
        tracing::info!("=== Scraping category: {} ===", category);

        match scraper.scrape_category_paginated(category, max_pages).await {
            Ok(listings) => {
                let category_listings: Vec<models::CompetitorListing> = listings
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
                    "Category {}: scraped {} valid listings",
                    category,
                    category_listings.len()
                );
                total_listings += category_listings.len();
                all_competitor_listings.extend(category_listings);
            }
            Err(e) => {
                tracing::error!("Failed to scrape category {}: {}", category, e);
            }
        }
    }

    tracing::info!(
        "Total scraped listings across all categories: {}",
        total_listings
    );

    // Send all competitor listings to Phoenix Mall
    if !all_competitor_listings.is_empty() {
        match phoenix.send_batch_intel(all_competitor_listings).await {
            Ok(_) => tracing::info!("All data sent to Phoenix Mall successfully"),
            Err(e) => tracing::error!("Failed to send data to Phoenix Mall: {}", e),
        }
    }

    tracing::info!("Spider shutdown complete");
    Ok(())
}
