use std::env;
use std::future::Future;
use reqwest::Error;
use scraper::{Html, Selector};
use tokio::sync::Mutex;
use std::sync::Arc;


async fn fetch_url(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn parse_html(html: &str) -> Vec<String>{
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();
    let mut links = vec![];
    for element in document.select(&selector) {
        let link = element.value().attr("href").unwrap_or("");
        links.push(link.to_string());
        println!("Link: {}", link)
    }
    links
}

async fn scrape_multiple_urls(urls: Vec<String>, depth: usize)  {
    let data = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];
    let check_urls = Arc::new(Mutex::new(vec![]));
    for url in urls {
        let data = Arc::clone(&data);
        let link_arr = Arc::clone(&check_urls);
        let handle = tokio::spawn(async move {
            if let Ok(html) = fetch_url(&url).await {
                let links: Vec<String> = parse_html(&html);
                link_arr.lock().await.push(links);
                data.lock().await.push(html);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    if depth > 0 {
        let links_to_scrape = check_urls.lock().await.clone();
        for links in links_to_scrape {
            Box::pin(scrape_multiple_urls(links, depth -1).await);
        }
    }
    Box::pin(async {})
}

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://dakshk.xyz".to_string(),
    ];
    let args: Vec<String> = env::args().collect();
    let depth: usize = match args.get(1).and_then(|s| s.parse().ok()) {
        Some(d) => d,
        None  => {
            eprintln!("Usage: cargo run <depth>");
            return;
        }
    };
    let _counter = Arc::new(depth);
    scrape_multiple_urls(urls, depth).await;
}
