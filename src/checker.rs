use log::warn;
use reqwest::{Client, Url};

pub async fn check_links_concurrently(client: &Client, links: &[Url]) -> Vec<(Url, Option<reqwest::StatusCode>)> {
    let checks = links.iter().map(|link| {
        let client = client.clone();
        let link = link.clone();
        tokio::spawn(async move {
            match client.head(link.clone()).send().await {
                Ok(resp) => (link, Some(resp.status())),
                Err(e) => {
                    warn!("Error checking {}: {}", link, e);
                    (link, None)
                }
            }
        })
    });

    let results = futures::future::join_all(checks).await;
    results.into_iter().map(|res| res.unwrap()).collect()
}

pub fn print_results(results: &[(Url, Option<reqwest::StatusCode>)], base_url: &str) {
    // Print detailed information for each link before printing the summary
    for (url, status_option) in results {
        match status_option {
            Some(status) if status.is_success() => {
                println!("{} : {}", url, status);
            }
            Some(status) => {
                // Append 'Invalid'
                println!("{} : {} Invalid", url, status);
            }
            None => {
                println!("{} : Error checking link", url);
            }
        }
    }

    // After printing details, print the summary as before.
    let total = results.len();
    let valid_count = results.iter().filter(|(_, status)| {
        if let Some(s) = status {
            s.is_success()
        } else {
            false
        }
    }).count();
    let invalid_count = total - valid_count;

    log::info!(
        "[Summary for {}]: {} total links, {} valid, {} invalid",
        base_url,
        total,
        valid_count,
        invalid_count
    );

    println!(
        "[Summary for {}]: {} total links, {} valid, {} invalid",
        base_url,
        total,
        valid_count,
        invalid_count
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;
    use reqwest::Url;
    use tokio;

    #[tokio::test]
    async fn test_check_links_concurrently() {
        let client = Client::new();
        let urls = vec![
            Url::parse("https://www.rust-lang.org").unwrap(),
        ];

        let results = check_links_concurrently(&client, &urls).await;
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_print_results() {
        let urls = vec![
            (Url::parse("https://example.com").unwrap(), Some(reqwest::StatusCode::OK)),
            (Url::parse("https://example.org").unwrap(), None),
        ];
        print_results(&urls, "https://test-base.com");
    }
}