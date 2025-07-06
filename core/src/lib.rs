pub fn add(left: usize, right: usize) -> usize {
    left + right
}

use anyhow::Result;
use bytes::Bytes;
use hyper::{body::to_bytes, Client};
use hyper::client::HttpConnector;
use hyper::http::uri::Uri;

/// Fetches the contents of the given URL and returns the response body as [`bytes::Bytes`].
///
/// This is a thin convenience wrapper around `hyper`'s [`Client`]. It is intended purely
/// for **demonstration / smoke-test** purposes in Phase 1 and will be replaced with a
/// more sophisticated connection-pooled client in later phases.
pub async fn fetch_url(url: &str) -> Result<Bytes> {
    let uri: Uri = url.parse()?;
    let client: Client<HttpConnector> = Client::new();

    let resp = client.get(uri).await?;
    let body = to_bytes(resp.into_body()).await?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn http_fetch_smoketest() {
        let rt = Runtime::new().expect("failed to build tokio runtime");
        let body = rt.block_on(async {
            fetch_url("https://httpbin.org/get").await.unwrap()
        });
        // The response should contain a JSON object with a "url" field.
        let text = String::from_utf8_lossy(&body);
        assert!(text.contains("httpbin.org/get"));
    }
}
