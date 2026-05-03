use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub phoenix_api_url: String,
    pub phoenix_api_token: String,
    pub _request_delay_ms: u64,
    pub _max_concurrent_requests: usize,
    pub user_agent: String,
    pub scrape_categories: Vec<String>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv().ok();

        let scrape_categories = vec![
            "mobile-phones".to_string(),
            "tablets".to_string(),
            "smart-watches".to_string(),
            "computers-and-laptops".to_string(),
            "tv-dvd-equipment".to_string(),
            "video-games-and-consoles".to_string(),
            "audio-and-music-equipment".to_string(),
            "headphones".to_string(),
            "computer-monitors".to_string(),
            "computer-hardware".to_string(),
            "computer-accessories".to_string(),
            "videogames".to_string(),
            "mens-fashion".to_string(),
            "womens-fashion".to_string(),
            "baby-kids-fashion".to_string(),
            "cars".to_string(),
            "real-estate".to_string(),
        ];

        Ok(Self {
            phoenix_api_url: env::var("PHOENIX_API_URL")
                .unwrap_or_else(|_| "http://localhost:5150".to_string()),
            phoenix_api_token: env::var("PHOENIX_API_TOKEN")
                .expect("PHOENIX_API_TOKEN must be set"),
            _request_delay_ms: env::var("REQUEST_DELAY_MS")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()?,
            _max_concurrent_requests: env::var("MAX_CONCURRENT_REQUESTS")
                .unwrap_or_else(|_| "3".to_string())
                .parse()?,
            user_agent: env::var("USER_AGENT").unwrap_or_else(|_| {
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string()
            }),
            scrape_categories,
        })
    }
}
