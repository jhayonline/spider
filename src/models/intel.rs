use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketIntel {
    pub product_id: Uuid,
    pub product_title: String,
    pub seller_current_price: Decimal,
    pub market_average_price: Decimal,
    pub market_median_price: Decimal,
    pub market_lowest_price: Decimal,
    pub market_highest_price: Decimal,
    pub competitor_count: usize,
    pub percentile_25: Decimal,
    pub percentile_75: Decimal,
    pub recommendation: PriceRecommendation,
    pub analyzed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PriceRecommendation {
    LowerTo(Decimal),
    KeepCurrent,
    NotEnoughData,
}

impl PriceRecommendation {
    pub fn message(&self) -> String {
        match self {
            PriceRecommendation::LowerTo(price) => {
                format!(
                    "Consider lowering your price to GHS {} to match average.",
                    price
                )
            }

            PriceRecommendation::KeepCurrent => {
                "Your price is competitive. No actions needed.".to_string()
            }

            PriceRecommendation::NotEnoughData => {
                "Not enough market data for recommendation yet.".to_string()
            }
        }
    }
}
