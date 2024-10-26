
use reqwest::Error;


fn fetch_url(url: &str) -> Result<String, Error> {
    let response = reqwest.get(url).await?;
    let body = response.text.await()?;
    Ok(body)
}
