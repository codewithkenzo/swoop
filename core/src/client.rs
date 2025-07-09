//! High-performance HTTP client with connection pooling and telemetry.

use http_body_util::{BodyExt, Empty};
use hyper::body::Bytes;
use hyper_util::client::legacy::{connect::HttpConnector, Client};
use hyper_util::rt::TokioExecutor;
use std::time::Duration;
use tokio::time::timeout;

use anyhow::{Context, Result};

// Create a new client with a connection pool.
pub fn new_client() -> Client<HttpConnector, Empty<Bytes>> {
    Client::builder(TokioExecutor::new())
        .pool_idle_timeout(Duration::from_secs(30))
        .build_http()
}

/// Fetches a URL using the optimized client with a timeout.
pub async fn fetch_with_timeout(
    client: &Client<HttpConnector, Empty<Bytes>>,
    url: &str,
    request_timeout: Duration,
) -> Result<Bytes> {
    let req = hyper::Request::builder()
        .uri(url)
        .body(Empty::new())
        .context("Failed to build request")?;

    let future = client.request(req);

    match timeout(request_timeout, future).await {
        Ok(Ok(response)) => {
            let body_bytes = response.into_body().collect().await?.to_bytes();
            Ok(body_bytes)
        }
        Ok(Err(e)) => Err(anyhow::anyhow!("HTTP request failed: {}", e)),
        Err(_) => Err(anyhow::anyhow!(
            "Request timed out after {:?}",
            request_timeout
        )),
    }
}
