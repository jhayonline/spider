use anyhow::Result;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

mod clients;
mod config;
mod models;

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

    // TODO: Scraping logic

    tracing::info!("Spider shutdown complete");
    Ok(())
}
