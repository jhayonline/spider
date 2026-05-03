# Spider - Market Intelligence Scraper for Phoenix Mall

## What This Is

Spider is a standalone web scraping and market intelligence service written in Rust. It collects pricing data from Jiji.com.gh to provide market insights for Phoenix Mall, a multi-vendor online marketplace.

## How It Works

1. Scrapes product listings from Jiji.com.gh across 17 categories
2. Extracts product titles, prices, conditions, and locations
3. Sends competitor data to Phoenix Mall via API
4. Phoenix Mall displays price insights to sellers

## Categories Scraped

- mobile-phones → Smartphones
- tablets → Tablets
- smart-watches → Smartwatches
- computers-and-laptops → Laptops
- tv-dvd-equipment → Televisions
- video-games-and-consoles → Video Games
- audio-and-music-equipment → Audio Equipment
- headphones → Headphones
- computer-monitors → Monitors
- computer-hardware → Computer Components
- computer-accessories → Computer Accessories
- videogames → Video Games
- mens-fashion → Men's Fashion
- womens-fashion → Women's Fashion
- baby-kids-fashion → Kids' Fashion
- cars → Cars
- real-estate → Real Estate

## Performance

| Metric             | Value            |
| ------------------ | ---------------- |
| Categories         | 17               |
| Pages per category | 5 (configurable) |
| Listings per run   | ~2,040           |
| Scraping time      | ~2.5-3 minutes   |
| Upload time        | ~1.5 minutes     |
| Total runtime      | ~4-5 minutes     |

## Setup

### Prerequisites

- Rust 1.70+
- Phoenix Mall backend running

### Getting the Phoenix API Token

1. Access your Phoenix Mall database:

```bash
psql -U postgres -d phoenix
```

1. Create a service account for the scraper:

```sql
INSERT INTO users (
    pid, email, password, api_key, name, role, is_active, email_verified_at, created_at, updated_at
) VALUES (
    gen_random_uuid(),
    'scraper@phoenixmall.com',
    'service_account_no_password',
    'scraper_' || gen_random_uuid(),
    'Market Intelligence Scraper',
    'admin',
    true,
    NOW(),
    NOW(),
    NOW()
);
```

1. Get the API key:

```sql
SELECT api_key FROM users WHERE email = 'scraper@phoenixmall.com';
```

The API key will look like: `scraper_21160b44-a380-4f6d-8e34-a29a3bb81e8e`

### Installation

```bash
git clone git@github.com:jhayonline/spider.git
cd spider
cp .env.example .env
```

### Configuration

Update `.env` file with your values:

```bash
PHOENIX_API_URL=http://localhost:5150
PHOENIX_API_TOKEN=scraper_21160b44-a380-4f6d-8e34-a29a3bb81e8e
MAX_PAGES_PER_CATEGORY=5
```

### Run

```bash
cargo build --release
cargo run
```

## Directory Structure

```
src/
├── analytics/price_engine.rs    # Statistical analysis
├── categories.rs                # Category mapping
├── clients/
│   ├── jiji.rs                  # Jiji scraper
│   └── phoenix.rs               # Phoenix Mall API client
├── config.rs                    # Configuration
├── main.rs                      # Orchestrator
└── models/competitor.rs         # Data structures
```

## Systemd Deployment

Create `/etc/systemd/system/spider.service`:

```ini
[Unit]
Description=Spider Market Intelligence Scraper
After=network.target

[Service]
Type=oneshot
User=your-user
WorkingDirectory=/path/to/spider
ExecStart=/path/to/spider/target/release/spider

[Install]
WantedBy=multi-user.target
```

Create `/etc/systemd/system/spider.timer`:

```ini
[Unit]
Description=Run Spider every 6 hours

[Timer]
OnCalendar=*-*-* 00,06,12,18:00
Persistent=true

[Install]
WantedBy=timers.target
```

Enable and start:

```bash
sudo systemctl daemon-reload
sudo systemctl enable spider.timer
sudo systemctl start spider.timer
```

## Check Logs

```bash
# View latest run
sudo journalctl -u spider.service -n 50

# Follow live
sudo journalctl -u spider.service -f

# Check timer status
sudo systemctl status spider.timer
```

## Technology

- Rust
- Reqwest (HTTP client)
- Scraper (HTML parsing)
- Tokio (async runtime)
- Serde (serialization)

```

```
