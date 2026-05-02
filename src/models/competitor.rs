use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitorPrice {
    pub product_title: String,
    pub price: Decimal,
    pub condition: String,
    pub platform: String,
    pub location: String,
    pub url: String,
    pub posted_at: DateTime<Utc>,
    pub scraped_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedListing {
    pub title: String,
    pub price: Option<Decimal>,
    pub condition: Option<String>,
    pub location: Option<String>,
    pub url: String,
}
