use std::env;
use std::collections::HashSet;
use reqwest::{Client, Error};
use scraper::{Html, Selector};
use tokio::sync::Mutex;
use std::sync::Arc;


async fn fetch_url(client: &Client, url: &str) -> Result<String, Error> {
    let response = client.get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn parse_html(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();
    document.select(&selector).filter_map(|element| element.value().attr("href")).map(|link| link.to_string()).collect()
}

async fn scrape_multiple_urls(client: Arc<Client>, url: String, depth: usize, max_depth: usize, visited: Arc<Mutex<HashSet<String>>>) {
    if depth > max_depth {
        return;
    }

    {
        let mut visited = visited.lock().await;
        if visited.contains(&url) {
            return;
        }
        visited.insert(url.clone());

    }

    if let Ok(html) = fetch_url(&client, &url).await {
        println!("Link(Depth: {}): {}", depth, url);
        let links = parse_html(&html);

        let mut tasks = vec![];
        for link in links {
            let client = Arc::clone(&client);
            let visited = Arc::clone(&visited);
            let task = tokio::spawn( async move {
               scrape_multiple_urls(client, link, depth + 1, max_depth, visited).await; 
            });
            tasks.push(task);
        }
        for task in tasks {
            let _ = task.await;
        }

    }
    
    //let data = Arc::new(Mutex::new(vec![]));
    //let mut handles = vec![];
    //
    //for url in urls {
    //    let data = Arc::clone(&data);
    //    let handle = tokio::spawn(async move {
    //        if let Ok(html) = fetch_url(&url).await {
    //            let urls: Vec<String> = parse_html(&html);
    //            data.lock().await.push(html);
    //        }
    //    });
    //    handles.push(handle);
    //}
    //for handle in handles {
    //    let _ = handle.await;
    //}
}

#[tokio::main]
async fn main() {
    let url = "https://dakshk.xyz".to_string();
    let client = Arc::new(Client::new());
    let visited = Arc::new(Mutex::new(HashSet::new()));
    let args: Vec<String> = env::args().collect();
    let depth: usize = args[1].parse().unwrap();
    scrape_multiple_urls(client, url, 0, depth, visited).await;
}
