use reqwest;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://jiji.com.gh/mobile-phones";

    let response = reqwest::get(url).await?.text().await?;

    let document = Html::parse_document(&response);

    let title_selector = Selector::parse("title").unwrap();

    let title = document
        .select(&title_selector)
        .next()
        .map(|el| el.inner_html())
        .unwrap_or("No title found".to_string());

    println!("TITLE: {}", title);

    Ok(())
}
