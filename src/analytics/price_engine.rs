use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PriceStatistics {
    pub mean: Decimal,
    pub median: Decimal,
    pub min: Decimal,
    pub max: Decimal,
    pub percentile_25: Decimal,
    pub percentile_75: Decimal,
    pub count: usize,
    pub outliers_removed: usize,
}

pub struct PriceEngine;

impl PriceEngine {
    /// Calculate price statistics from a list of prices
    pub fn calculate_statistics(prices: &[Decimal]) -> Option<PriceStatistics> {
        if prices.is_empty() {
            return None;
        }

        let mut sorted_prices = prices.to_vec();
        sorted_prices.sort();

        let count = sorted_prices.len();

        // Calculate mean
        let sum: Decimal = sorted_prices.iter().sum();
        let mean = sum / Decimal::from_usize(count).unwrap();

        // Calculate median
        let median = if count % 2 == 0 {
            (sorted_prices[count / 2 - 1] + sorted_prices[count / 2]) / Decimal::from_u8(2).unwrap()
        } else {
            sorted_prices[count / 2]
        };

        // Calculate percentiles
        let percentile_25 = Self::percentile(&sorted_prices, 25);
        let percentile_75 = Self::percentile(&sorted_prices, 75);

        Some(PriceStatistics {
            mean,
            median,
            min: sorted_prices[0],
            max: sorted_prices[count - 1],
            percentile_25,
            percentile_75,
            count,
            outliers_removed: 0,
        })
    }

    /// Calculate statistics with outlier removal (prices beyond 3 standard deviations)
    pub fn calculate_statistics_no_outliers(prices: &[Decimal]) -> Option<PriceStatistics> {
        if prices.is_empty() {
            return None;
        }

        // First pass, calculate mean and std deviation
        let stats = Self::calculate_statistics(prices)?;

        if stats.count < 4 {
            return Some(stats);
        }

        // Calculate standard deviation
        let variance: Decimal = prices
            .iter()
            .map(|p| {
                let diff = *p - stats.mean;
                diff * diff
            })
            .sum::<Decimal>()
            / Decimal::from_usize(stats.count).unwrap();

        let std_dev = Self::decimal_sqrt(variance);

        // Filter out outliers (beyond 3 standard deviations)
        let three_std = std_dev * Decimal::from_u8(3).unwrap();
        let filtered_prices: Vec<Decimal> = prices
            .iter()
            .filter(|p| {
                let diff = if **p > stats.mean {
                    **p - stats.mean
                } else {
                    stats.mean - **p
                };
                diff <= three_std
            })
            .cloned()
            .collect();

        let outliers_removed = prices.len() - filtered_prices.len();

        if filtered_prices.is_empty() {
            return Some(stats);
        }

        let mut filtered_stats = Self::calculate_statistics(&filtered_prices)?;
        filtered_stats.outliers_removed = outliers_removed;

        Some(filtered_stats)
    }

    /// Calculate percentile (0-100)
    fn percentile(sorted_prices: &[Decimal], percentile: u8) -> Decimal {
        if sorted_prices.is_empty() {
            return Decimal::ZERO;
        }

        let index = (percentile as f64 / 100.0) * (sorted_prices.len() - 1) as f64;
        let lower_index = index.floor() as usize;
        let upper_index = index.ceil() as usize;

        if lower_index == upper_index {
            sorted_prices[lower_index]
        } else {
            let weight = Decimal::from_f64(index - lower_index as f64).unwrap();
            sorted_prices[lower_index]
                + (sorted_prices[upper_index] - sorted_prices[lower_index]) * weight
        }
    }

    /// Group prices by product type (simple keyword matching)
    pub fn group_by_product_type(
        listings: &[crate::models::ScrapedListing],
    ) -> HashMap<String, Vec<Decimal>> {
        let mut groups: HashMap<String, Vec<Decimal>> = HashMap::new();

        for listing in listings {
            if let Some(price) = listing.price {
                // Extract product model (simple approach - first few words)
                let product_type = listing
                    .title
                    .split_whitespace()
                    .take(3)
                    .collect::<Vec<&str>>()
                    .join(" ");

                groups
                    .entry(product_type)
                    .or_insert_with(Vec::new)
                    .push(price);
            }
        }

        groups
    }

    /// Compare seller price against market statistics
    pub fn compare_to_market(
        seller_price: Decimal,
        market_stats: &PriceStatistics,
    ) -> MarketComparison {
        let diff = seller_price - market_stats.mean;
        let percent_diff = (diff / market_stats.mean) * Decimal::from_u8(100).unwrap();

        if seller_price > market_stats.percentile_75 {
            MarketComparison::AboveAverage(percent_diff, market_stats.percentile_75)
        } else if seller_price < market_stats.percentile_25 {
            MarketComparison::BelowAverage(percent_diff, market_stats.percentile_25)
        } else {
            MarketComparison::Competitive(percent_diff)
        }
    }

    fn decimal_sqrt(decimal: Decimal) -> Decimal {
        let float_val = decimal.to_f64().unwrap_or(0.0);
        let sqrt_val = float_val.sqrt();

        Decimal::from_f64(sqrt_val).unwrap_or(Decimal::ZERO)
    }
}

#[derive(Debug, Clone)]
pub enum MarketComparison {
    AboveAverage(Decimal, Decimal), // percent above, suggested price
    BelowAverage(Decimal, Decimal), // percent below, market low
    Competitive(Decimal),           // percent difference from mean
}

impl MarketComparison {
    pub fn explanation(&self) -> String {
        match self {
            MarketComparison::AboveAverage(pct, suggested) => {
                format!(
                    "Your price is {:.2}% above market average. Consider pricing around GHS {} to be competitive.",
                    pct, suggested
                )
            }
            MarketComparison::BelowAverage(pct, market_low) => {
                format!(
                    "Your price is {:.2}% below market average (very competitive!). You could potentially increase to GHS {} and still sell well.",
                    pct.abs(),
                    market_low
                )
            }
            MarketComparison::Competitive(pct) => {
                format!(
                    "Your price is within {:.2}% of market average. Well positioned!",
                    pct.abs()
                )
            }
        }
    }

    pub fn recommended_price(&self) -> Option<Decimal> {
        match self {
            MarketComparison::AboveAverage(_, suggested) => Some(*suggested),
            MarketComparison::BelowAverage(_, market_low) => Some(*market_low),
            MarketComparison::Competitive(_) => None,
        }
    }
}
