use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitorListing {
    pub product_title: String,
    pub price: Decimal,
    pub condition: Option<String>,
    pub platform: String,
    pub location: Option<String>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedListing {
    pub title: String,
    pub price: Option<Decimal>,
    pub condition: Option<String>,
    pub location: Option<String>,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceIntelData {
    pub product_id: uuid::Uuid,
    pub product_title: String,
    pub seller_current_price: Decimal,
    pub market_average_price: Decimal,
    pub market_median_price: Decimal,
    pub market_lowest_price: Decimal,
    pub market_highest_price: Decimal,
    pub competitor_count: usize,
    pub percentile_25: Decimal,
    pub percentile_75: Decimal,
    pub recommendation: String,
    pub analyzed_at: DateTime<Utc>,
}
