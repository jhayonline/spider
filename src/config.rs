use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub phoenix_api_url: String,
    pub phoenix_api_token: String,
    pub _request_delay_ms: u64,
    pub _max_concurrent_requests: usize,
    pub user_agent: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv().ok();

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
        })
    }
}
