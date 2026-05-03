use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PriceStatistics {
    pub mean: Decimal,
    pub median: Decimal,
    pub min: Decimal,
    pub max: Decimal,
    pub count: usize,
    pub outliers_removed: usize,
}

#[allow(dead_code)]
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

        Some(PriceStatistics {
            mean,
            median,
            min: sorted_prices[0],
            max: sorted_prices[count - 1],
            count,
            outliers_removed: 0,
        })
    }

    /// Calculate statistics with outlier removal (prices beyond 3 standard deviations)
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    fn decimal_sqrt(decimal: Decimal) -> Decimal {
        let float_val = decimal.to_f64().unwrap_or(0.0);
        let sqrt_val = float_val.sqrt();

        Decimal::from_f64(sqrt_val).unwrap_or(Decimal::ZERO)
    }
}
