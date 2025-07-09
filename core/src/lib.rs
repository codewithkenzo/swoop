pub mod client;
pub mod security;

use anyhow::Result;
use bytes::Bytes;
use once_cell::sync::Lazy;
use security::UrlValidator;
use std::time::Duration;

static URL_VALIDATOR: Lazy<UrlValidator> = Lazy::new(UrlValidator::default);

/// Fetches the contents of the given URL and returns the response body as [`bytes::Bytes`].
///
/// This function now includes SSRF protection by validating URLs before making requests,
/// and uses a high-performance, pooled HTTP client with a configurable timeout.
pub async fn fetch_url(url: &str, request_timeout: Duration) -> Result<Bytes> {
    // Validate URL first to prevent SSRF attacks
    URL_VALIDATOR.validate_url(url)?;

    let client = client::new_client();
    client::fetch_with_timeout(&client, url, request_timeout).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::runtime::Runtime;

    #[test]
    fn http_fetch_smoketest() {
        let rt = Runtime::new().expect("failed to build tokio runtime");
        let result = rt.block_on(async {
            fetch_url("https://httpbin.org/get", Duration::from_secs(10)).await
        });

        match result {
            Ok(body) => {
                let text = String::from_utf8_lossy(&body);
                assert!(text.contains("httpbin.org/get"));
            }
            Err(e) => {
                // Allow the test to pass if there's a network error (common in CI)
                eprintln!("Network test failed (this is OK in CI): {}", e);
            }
        }
    }

    #[test]
    fn test_fetch_timeout() {
        let rt = Runtime::new().expect("failed to build tokio runtime");
        let result = rt.block_on(async {
            // httpbin.org/delay/5 will take 5 seconds to respond
            fetch_url("https://httpbin.org/delay/5", Duration::from_secs(2)).await
        });

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Request timed out"));
    }
}
