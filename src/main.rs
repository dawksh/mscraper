
use reqwest::Error;
use scraper::{Html, Selector};
use tokio::sync::Mutex;
use std::sync::Arc;


async fn fetch_url(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn parse_html(html: &str) {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();
    for element in document.select(&selector) {
        let link = element.value().attr("href").unwrap_or("");
        println!("Link: {}", link)
    }
}

async fn scrape_multiple_urls(urls: Vec<String>) {
    let data = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];

    for url in urls {
        let data = Arc::clone(&data);
        let handle = tokio::spawn(async move {
            if let Ok(html) = fetch_url(&url).await {
                parse_html(&html);
                data.lock().await.push(html);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }
}

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://dakshk.xyz".to_string(),
    ];

    scrape_multiple_urls(urls).await;
}
