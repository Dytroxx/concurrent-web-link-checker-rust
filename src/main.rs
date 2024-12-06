mod config;
mod fetcher;
mod checker;

use clap::Parser;
use config::Config;
use log::{error, info};
use reqwest::Client;
use tokio::task;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let args = config::Cli::parse();

    // Read and parse url list file
    let config_data = std::fs::read_to_string(&args.config)?;
    let config: Config = serde_json::from_str(&config_data)?;

    // Create a shared HTTP client with timeout
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let mut tasks = vec![];
    for url in config.urls {
        let client = client.clone();
        tasks.push(task::spawn(async move {
            match fetch_and_check_links(&client, &url).await {
                Ok(_) => {}
                Err(e) => error!("Error processing {}: {}", url, e),
            }
        }));
    }

    for handle in tasks {
        let _ = handle.await;
    }

    Ok(())
}

async fn fetch_and_check_links(client: &Client, url: &str) -> Result<(), Box<dyn Error>> {
    let base_url = reqwest::Url::parse(url)?;
    let links = fetcher::fetch_links(client, &base_url).await?;
    info!("Found {} links on {}", links.len(), url);

    let results = checker::check_links_concurrently(client, &links).await;
    checker::print_results(&results, url);

    Ok(())
}