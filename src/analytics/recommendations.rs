use crate::analytics::price_engine::{MarketComparison, PriceEngine, PriceStatistics};
use crate::models::intel::{MarketIntel, PriceRecommendation};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use uuid::Uuid;

pub struct RecommendationEngine;

impl RecommendationEngine {
    /// Generate market intelligence from competitor data
    pub fn generate_intel(
        product_id: Uuid,
        product_title: String,
        seller_price: Decimal,
        competitor_prices: &[Decimal],
    ) -> MarketIntel {
        // Calculate market statistics (with outlier removal)
        let stats = match PriceEngine::calculate_statistics_no_outliers(competitor_prices) {
            Some(s) => s,
            None => {
                return MarketIntel {
                    product_id,
                    product_title,
                    seller_current_price: seller_price,
                    market_average_price: seller_price,
                    market_median_price: seller_price,
                    market_lowest_price: seller_price,
                    market_highest_price: seller_price,
                    competitor_count: 0,
                    percentile_25: seller_price,
                    percentile_75: seller_price,
                    recommendation: PriceRecommendation::NotEnoughData,
                    analyzed_at: chrono::Utc::now(),
                };
            }
        };

        // Compare seller price to market
        let comparison = PriceEngine::compare_to_market(seller_price, &stats);

        // Generate recommendation
        let recommendation = if let Some(recommended) = comparison.recommended_price() {
            if recommended < seller_price {
                PriceRecommendation::LowerTo(recommended)
            } else {
                PriceRecommendation::KeepCurrent
            }
        } else {
            PriceRecommendation::KeepCurrent
        };

        MarketIntel {
            product_id,
            product_title,
            seller_current_price: seller_price,
            market_average_price: stats.mean,
            market_median_price: stats.median,
            market_lowest_price: stats.min,
            market_highest_price: stats.max,
            competitor_count: stats.count,
            percentile_25: stats.percentile_25,
            percentile_75: stats.percentile_75,
            recommendation,
            analyzed_at: chrono::Utc::now(),
        }
    }

    /// Generate simple price insight (for quick display)
    pub fn quick_insight(
        seller_price: Decimal,
        market_avg: Decimal,
        competitor_count: usize,
    ) -> String {
        if competitor_count == 0 {
            return "Not enough market data for analysis".to_string();
        }

        let diff = seller_price - market_avg;
        let hundred = Decimal::from_u8(100).unwrap();
        let percent = (diff / market_avg * hundred).round_dp(1);

        if competitor_count < 3 {
            format!(
                "Limited data ({} competitors). Market average: GHS {}",
                competitor_count, market_avg
            )
        } else if seller_price > market_avg {
            format!("{:.1}% above market average (GHS {})", percent, market_avg)
        } else if seller_price < market_avg {
            format!(
                "{:.1}% below market average (GHS {})",
                percent.abs(),
                market_avg
            )
        } else {
            format!("At market average (GHS {})", market_avg)
        }
    }
}
