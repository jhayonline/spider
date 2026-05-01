# Spider - Market Intelligence Scraper for Phoenix Mall

## What This Is

Spider is a standalone web scraping and market intelligence service written in Rust. It collects pricing data, product information, and market trends from competitor marketplaces (starting with jiji.com.gh) to provide actionable insights for Phoenix Mall, a multi-vendor online marketplace.

## Problems It Solves

Phoenix Mall sellers face several challenges that Spider addresses:

1. Pricing Blindness - Sellers don't know what similar products sell for on other platforms, leading to overpriced listings that never sell or underpriced listings that leave money on the table.

2. Manual Market Research - Without automation, sellers would need to manually check competitor sites daily to stay competitive.

3. Slow Inventory Turnover - Products priced above market average take longer to sell, reducing platform transaction volume.

4. Seller Churn - Frustrated sellers who can't sell their items may leave the platform permanently.

## How It Contributes to Phoenix Mall

Spider enhances Phoenix Mall in four key ways:

1. Seller Empowerment
   - Shows sellers real-time market prices for similar items
   - Provides price recommendations based on actual competitor data
   - Helps sellers price competitively without guesswork

2. Platform Intelligence
   - Identifies trending product categories missing from Phoenix Mall
   - Detects demand signals before competitors do
   - Informs category expansion decisions

3. Buyer Experience
   - Competitive pricing across the platform attracts more buyers
   - Faster listing turnover means fresher inventory
   - Better deals keep buyers returning

4. Operational Efficiency
   - Automated market monitoring reduces manual analysis work
   - Data-driven decisions replace guesswork
   - Scalable intelligence without headcount growth

## How It Works

The scraper operates as a scheduled background service with this workflow:

Phase 1 - Data Collection

- Fetches product listings from competitor sites using HTTP requests
- Parses HTML to extract product titles, prices, conditions, and locations
- Handles pagination to collect complete market data

Phase 2 - Analysis

- Maps competitor products to Phoenix Mall categories
- Calculates market averages, percentiles, and price distributions
- Detects outliers, trends, and demand signals

Phase 3 - Intelligence Delivery

- Compares seller prices against market data
- Generates price recommendations using statistical analysis
- Pushes insights to Phoenix Mall via its existing API
- Creates notifications for sellers when price adjustments are needed

Phase 4 - Continuous Learning

- Stores market intelligence in Phoenix Mall database
- Tracks price trends over time
- Improves recommendation accuracy with historical data

## Architecture

The scraper is intentionally decoupled from Phoenix Mall, running as an independent binary. This provides:

- Isolation - Scraper crashes don't affect marketplace operations
- Independent scaling - Can run on different schedules or hardware
- Clean dependencies - No framework overhead for a scheduled job
- Simple deployment - Single binary run by cron or systemd timer

## Directory Structure Explained

```text
src/
├── analytics/              # Market analysis and intelligence generation
│   ├── mod.rs             # Module exports and public interface
│   ├── price_engine.rs    # Statistical price analysis, market averages, percentile calculations
│   └── recommendations.rs # Seller-facing recommendations and insight formatting
│
├── clients/               # External service integrations
│   ├── mod.rs             # Client module exports
│   ├── jiji.rs            # Jiji.com.gh scraper - HTML parsing, pagination, data extraction
│   └── phoenix.rs         # Phoenix Mall API client - authentication, product fetching, notification sending
│
├── config.rs              # Configuration management - environment variables, API keys, schedules
│
├── main.rs                # Orchestrator - ties everything together, runs the scraping workflow
│
└── models/                # Data structures shared across the system
    ├── mod.rs             # Model exports
    ├── competitor.rs      # CompetitorPrice struct - represents scraped product data
    └── intel.rs           # MarketIntel struct - represents analyzed insights and recommendations
```

## Data Flow

1. Config loads environment variables (API keys, URLs, schedules)
2. Main orchestrator authenticates with Phoenix Mall
3. Client fetches active product listings from Phoenix Mall
4. For each product, competitor data is scraped from external sites
5. Price engine analyzes market data and generates intelligence
6. Recommendations are formatted and sent back to Phoenix Mall via API
7. Phoenix Mall creates notifications for affected sellers

## Technology Stack

- Rust - Memory-safe, high-performance systems language
- Reqwest - HTTP client with cookie and gzip support
- Scraper - HTML parsing and CSS selector extraction
- Tokio - Async runtime for concurrent scraping
- Serde - Serialization for Phoenix Mall API communication
- Chrono - Date/time handling for price trends
- Rust Decimal - Precise currency calculations

## Planned Competitor Support

- Phase 1 - Jiji.com.gh (primary Ghana marketplace)
- Phase 2 - Tonaton.com (secondary Ghana marketplace)
- Phase 3 - OLX (regional marketplace)
- Phase 4 - Facebook Marketplace (social commerce)

## Contribution to Phoenix Mall's Bottom Line

By helping sellers price competitively, Spider directly impacts:

- Higher transaction velocity (faster sales)
- Increased platform trust (fair prices)
- Reduced abandoned listings (priced to sell)
- Improved seller retention (successful sales)
- Better buyer value perception (competitive marketplace)

The intelligence gathered also guides strategic decisions about which new categories to add, where demand is growing, and how Phoenix Mall should position itself against competitors.
