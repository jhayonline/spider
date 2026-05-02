use anyhow::Result;
use tracing_subscriber;

mod clients;
mod config;
mod models;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("spider=debug")
        .init();

    tracing::info!("Starting Spider Intelligence Engine");

    let config = config::Config::from_env()?;
    tracing::debug!("Configuration Loaded: {:?}", config);

    tracing::info!("Spider Shutdown Complete");
    Ok(())
}
