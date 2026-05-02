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
