use log::warn;
use reqwest::{Client, Url};
use scraper::{Html, Selector};
use std::error::Error;

pub async fn fetch_links(client: &Client, base_url: &Url) -> Result<Vec<Url>, Box<dyn Error>> {
    let resp = client.get(base_url.clone()).send().await?;
    if !resp.status().is_success() {
        return Err(format!("Non-success status {} from {}", resp.status(), base_url).into());
    }

    let body = resp.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("a").unwrap();

    let mut links = Vec::new();
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            match base_url.join(href) {
                Ok(full_url) => links.push(full_url),
                Err(e) => {
                    warn!("Failed to resolve relative link '{}': {}", href, e);
                }
            }
        }
    }
    Ok(links)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Url;
    use tokio;

    // Note: because `fetch_links` is async, use `tokio::test`
    #[tokio::test]
    async fn test_fetch_links_with_valid_url() {
        let client = Client::new();
        let url = Url::parse("https://www.rust-lang.org").unwrap();

        let result = fetch_links(&client, &url).await;
        assert!(result.is_ok());
        let links = result.unwrap();

        assert!(!links.is_empty());
    }

    #[tokio::test]
    async fn test_fetch_links_with_invalid_url() {
        let client = Client::new();
        let url = Url::parse("https://non-existent-url.com").unwrap();

        let result = fetch_links(&client, &url).await;
        assert!(result.is_err());
    }
}