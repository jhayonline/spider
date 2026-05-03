# Spider - Market Intelligence Scraper Development Plan

## Milestone 1: Project Foundation

**Goal:** Working standalone binary that can scrape basic data from one source.

**Tasks:**

- Set up project structure with config, models, clients modules
- Implement configuration loading from .env file
- Create Phoenix Mall API client with authentication
- Implement basic Jiji scraper that extracts product listings from single page
- Parse product title, price, condition, location from HTML
- Store scraped data locally in JSON files for debugging
- Add logging with tracing

**Deliverables:**

- Binary runs without errors
- Successfully scrapes 10+ products from Jiji first page
- Scraped data validates against defined models
- README updated with usage instructions

**Testing:**

- Manual run on test category (mobile phones)
- Verify price parsing handles GHS format correctly
- Log output shows extracted data

## Milestone 2: Price Intelligence Engine

**Goal:** Analyze scraped data and generate actionable insights.

**Tasks:**

- Implement price statistics calculator (mean, median, percentiles)
- Add outlier detection to filter unrealistic prices
- Create recommendation engine with business rules
- Calculate market average by category matching
- Generate price recommendations (lower, keep, not enough data)
- Cache market data to avoid re-scraping same products

**Deliverables:**

- Takes scraped competitor data → returns MarketIntel struct
- Recommendation logic configurable via rules file
- Unit tests for price calculations
- Handles edge cases (missing prices, invalid formats)

**Testing:**

- Unit tests with sample competitor data
- Verify percentiles calculated correctly
- Test recommendation thresholds (e.g., >20% above market → lower)

## Milestone 3: Phoenix Mall Integration

**Goal:** Push intelligence to Phoenix Mall via its API.

**Tasks:**

- Implement Phoenix Mall API endpoint for receiving price intel (add to Phoenix Mall codebase)
- Create database tables for competitor_listings and price_recommendations
- Add authentication for scraper service account
- Spider calls API with MarketIntel data
- Phoenix Mall creates notifications for sellers with recommendations
- Add error handling for failed API calls (retry with backoff)

**Deliverables:**

- Spider successfully sends data to Phoenix Mall
- Notifications appear for sellers with price recommendations
- Database stores all scraped competitor data
- API returns proper success/error responses

**Testing:**

- Integration test with local Phoenix Mall instance
- Verify notifications created correctly
- Test retry logic with simulated API failures

## Milestone 4: Production Readiness

**Goal:** Reliable, scheduled scraping that handles real-world challenges.

**Tasks:**

- Add pagination to scrape multiple pages
- Implement rate limiting to avoid being blocked
- Add random delays and user agent rotation
- Handle CAPTCHA detection and graceful failure
- Implement resume capability (save progress, restart from where left off)
- Add metrics tracking (products scraped, success rate, API calls)
- Create systemd timer or cron configuration for scheduling
- Add health check endpoint for monitoring

**Deliverables:**

- Scrapes 100+ products across multiple pages
- Runs for 1+ hours without being blocked
- Schedules runs every 6 hours via systemd
- Logs metrics to file for monitoring
- Docker container for easy deployment

**Testing:**

- 24-hour test run with actual Jiji scraping
- Monitor for blocks or rate limiting
- Verify resume works after interruption
- Test with production Phoenix Mall instance

## Milestone 5: Expansion & Optimization

**Goal:** Add more data sources and advanced features.

**Tasks:**

- Add Tonaton.com scraper (same interface as Jiji)
- Implement parallel scraping with semaphore-based concurrency
- Add price trend detection (price drops over time)
- Implement demand signal detection (listing velocity)
- Add category gap analysis (categories present on competitors but missing from Phoenix Mall)
- Create simple dashboard (optional - can be separate project)

**Deliverables:**

- Supports 2+ competitor platforms
- Concurrent scraping reduces total runtime by 50%+
- Identifies trending products and categories
- Provides weekly market reports

**Testing:**

- Run with both Jiji and Tonaton simultaneously
- Verify data from different platforms stored separately
- Test concurrency doesn't trigger rate limits

## Success Criteria

**Milestone 2 Completion:**

- Spider can be independently verified to provide accurate price recommendations
- Manual review shows recommendations are reasonable

**Milestone 3 Completion:**

- At least one seller receives and acts on a price recommendation
- Transaction velocity increases for recommended products (measured after 2 weeks)

**Milestone 4 Completion:**

- Spider runs for 7 days without intervention
- Zero crashes or memory leaks
- Successfully processes 95%+ of targeted listings

**Milestone 5 Completion:**

- Covers 80%+ of product categories present on competitor sites
- Scraping completes within 2 hours (all platforms combined)

## Technical Debt & Risks

| Risk                                 | Mitigation                                                           |
| ------------------------------------ | -------------------------------------------------------------------- |
| Jiji changes HTML structure          | Add HTML structure versioning, alert on parse failures               |
| Getting blocked/rate limited         | Implement rotating proxies (future), respect robots.txt              |
| API changes in Phoenix Mall          | Version API endpoints, Spider checks compatibility on startup        |
| Memory leaks in long-running scraper | Run as scheduled job (exits after each run), not long-running daemon |
| Legal compliance                     | Respect robots.txt, add delays, don't overload servers               |

## Definition of Done for Each Milestone

- All tasks completed
- Tests pass
- Code reviewed
- Documentation updated
- Runs without errors in staging environment
- Known bugs documented
