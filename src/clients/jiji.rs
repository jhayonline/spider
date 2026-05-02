use crate::models::ScrapedListing;
use anyhow::Result;
use reqwest::Client;
use rust_decimal::Decimal;
use scraper::{Html, Selector};

pub struct JijiScraper {
    client: Client,
    base_url: String,
}

impl JijiScraper {
    pub fn new(user_agent: &str) -> Self {
        let client = Client::builder()
            .user_agent(user_agent)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            base_url: "https://jiji.com.gh".to_string(),
        }
    }

    /// Scrape a single page of listings
    #[allow(dead_code)]
    pub async fn scrape_category(&self, category_path: &str) -> Result<Vec<ScrapedListing>> {
        let url = format!("{}/{}", self.base_url, category_path);
        tracing::info!("Scraping category: {}", url);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch page: HTTP {}", response.status());
        }

        let html = response.text().await?;
        let document = Html::parse_document(&html);

        self.parse_listings(&document).await
    }

    /// Scrape multiple pages of listings
    pub async fn scrape_category_paginated(
        &self,
        category_path: &str,
        max_pages: usize,
    ) -> Result<Vec<ScrapedListing>> {
        let mut all_listings = Vec::new();

        for page in 1..=max_pages {
            let url = if page == 1 {
                format!("{}/{}", self.base_url, category_path)
            } else {
                format!("{}/{}/?page={}", self.base_url, category_path, page)
            };

            tracing::info!("Scraping page {} of {}: {}", page, max_pages, url);

            let response = self.client.get(&url).send().await?;

            if !response.status().is_success() {
                tracing::warn!("Failed to fetch page {}: HTTP {}", page, response.status());
                break;
            }

            let html = response.text().await?;
            let document = Html::parse_document(&html);

            let page_listings = self.parse_listings(&document).await?;

            if page_listings.is_empty() {
                tracing::info!("No more listings found on page {}, stopping", page);
                break;
            }

            tracing::info!("Page {}: scraped {} listings", page, page_listings.len());
            all_listings.extend(page_listings);

            // Be nice to the server - delay between pages
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

        Ok(all_listings)
    }

    /// Parse listings from HTML document
    async fn parse_listings(&self, document: &Html) -> Result<Vec<ScrapedListing>> {
        let listing_selector = Selector::parse(".b-list-advert__gallery__item").unwrap();
        let title_selector = Selector::parse(".b-advert-title-inner").unwrap();
        let price_selector = Selector::parse(".qa-advert-price").unwrap();
        let link_selector = Selector::parse("a.qa-advert-list-item").unwrap();

        let mut listings = Vec::new();

        for element in document.select(&listing_selector) {
            let title = element
                .select(&title_selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let price_text = element
                .select(&price_selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let price = Self::parse_price(&price_text);

            let url = element
                .select(&link_selector)
                .next()
                .and_then(|el| el.value().attr("href"))
                .map(|href| {
                    if href.starts_with("http") {
                        href.to_string()
                    } else {
                        format!("{}{}", self.base_url, href)
                    }
                })
                .unwrap_or_default();

            let condition_selector = Selector::parse(".b-list-advert-base__item-attr").unwrap();
            let condition = element
                .select(&condition_selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .filter(|text| !text.is_empty());

            let location_selector = Selector::parse(".b-list-advert__region__text").unwrap();
            let location = element
                .select(&location_selector)
                .next()
                .map(|el| el.text().collect::<String>());

            if !title.is_empty() {
                listings.push(ScrapedListing {
                    title,
                    price,
                    condition,
                    location,
                    url,
                });
            }
        }

        Ok(listings)
    }

    fn parse_price(price_text: &str) -> Option<Decimal> {
        // Remove GH₵ and any non-numeric characters except decimal and commas
        let cleaned: String = price_text
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '.' || *c == ',')
            .collect();

        // Replace comma with empty (GH₵ 2,350 -> 2350)
        let cleaned = cleaned.replace(',', "");

        if cleaned.is_empty() {
            None
        } else {
            cleaned.parse::<Decimal>().ok()
        }
    }
}
