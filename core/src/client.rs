//! High-performance HTTP client using reqwest.

use anyhow::Result;
use bytes::Bytes;
use reqwest::Client;
use std::time::Duration;

// Create a new reqwest client.
pub fn new_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap()
}

/// Fetches a URL using the reqwest client with a timeout.
pub async fn fetch_with_timeout(
    client: &Client,
    url: &str,
    request_timeout: Duration,
) -> Result<Bytes> {
    let response = client.get(url).timeout(request_timeout).send().await?;
    let bytes = response.bytes().await?;
    Ok(bytes)
}
