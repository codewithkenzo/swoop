use anyhow::Result;
use fantoccini::{Client, ClientBuilder, Locator};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use url::Url;

/// Configuration for browser automation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserConfig {
    /// Maximum number of concurrent browser instances
    pub max_instances: usize,
    /// Page load timeout in seconds
    pub page_timeout_secs: u64,
    /// WebDriver server URL (e.g., http://localhost:4444)
    pub webdriver_url: String,
    /// Whether to run in headless mode
    pub headless: bool,
    /// Custom user agent string
    pub user_agent: Option<String>,
    /// Browser window size
    pub window_size: (u32, u32),
    /// Additional browser capabilities
    pub capabilities: serde_json::Value,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        let mut caps = serde_json::Map::new();
        caps.insert(
            "browserName".to_string(),
            serde_json::Value::String("chrome".to_string()),
        );

        // Chrome-specific options
        let mut chrome_options = serde_json::Map::new();
        let args = [
            "--no-sandbox",
            "--disable-dev-shm-usage",
            "--disable-gpu",
            "--disable-web-security",
            "--disable-features=VizDisplayCompositor",
            "--headless=new", // Use new headless mode
        ];
        chrome_options.insert(
            "args".to_string(),
            serde_json::Value::Array(
                args.iter()
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .collect(),
            ),
        );

        caps.insert(
            "goog:chromeOptions".to_string(),
            serde_json::Value::Object(chrome_options),
        );

        Self {
            max_instances: 5,
            page_timeout_secs: 30,
            webdriver_url: "http://localhost:4444".to_string(),
            headless: true,
            user_agent: Some("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string()),
            window_size: (1920, 1080),
            capabilities: serde_json::Value::Object(caps),
        }
    }
}

/// Browser pool for managing multiple browser instances
pub struct BrowserPool {
    config: BrowserConfig,
    semaphore: Arc<Semaphore>,
}

impl BrowserPool {
    pub fn new(config: BrowserConfig) -> Self {
        let semaphore = Arc::new(Semaphore::new(config.max_instances));
        Self { config, semaphore }
    }

    /// Get a browser instance from the pool
    pub async fn get_browser(&self) -> Result<BrowserInstance> {
        let _permit = self.semaphore.acquire().await?;

        let mut client_builder = ClientBuilder::native();

        // Set capabilities
        if let serde_json::Value::Object(caps) = &self.config.capabilities {
            client_builder.capabilities(caps.clone());
        }

        let client = client_builder.connect(&self.config.webdriver_url).await?;

        // Set window size
        client
            .set_window_size(self.config.window_size.0, self.config.window_size.1)
            .await?;

        // Set user agent if specified
        if let Some(user_agent) = &self.config.user_agent {
            client.execute(
                &format!(r#"Object.defineProperty(navigator, 'userAgent', {{get: function(){{return '{}'}}}});"#, user_agent),
                vec![]
            ).await?;
        }

        // Don't drop the permit, keep it alive with the instance
        std::mem::forget(_permit);

        Ok(BrowserInstance {
            client: Arc::new(client),
            config: self.config.clone(),
            _semaphore: self.semaphore.clone(),
        })
    }
}

/// Wrapper around a browser instance with automatic cleanup
pub struct BrowserInstance {
    client: Arc<Client>,
    config: BrowserConfig,
    _semaphore: Arc<Semaphore>,
}

impl BrowserInstance {
    /// Navigate to a URL and extract content
    pub async fn scrape_page(&self, url: &str) -> Result<ScrapedContent> {
        let _parsed_url = Url::parse(url)?;

        // Navigate to the page
        self.client.goto(url).await?;

        // Wait for page to load
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Extract content
        let html = self.client.source().await?;
        let title = self.client.title().await.unwrap_or_default();
        let current_url = self.client.current_url().await?.to_string();

        // Take a screenshot for debugging (optional)
        let screenshot = if !self.config.headless {
            Some(self.client.screenshot().await?)
        } else {
            None
        };

        Ok(ScrapedContent {
            url: current_url,
            title,
            html,
            screenshot,
            timestamp: chrono::Utc::now(),
        })
    }

    /// Execute JavaScript on the page
    pub async fn execute_script(&self, script: &str) -> Result<serde_json::Value> {
        let result = self.client.execute(script, vec![]).await?;
        Ok(result)
    }

    /// Interact with page elements (click, type, etc.)
    pub async fn interact_with_page(
        &self,
        url: &str,
        actions: Vec<PageAction>,
    ) -> Result<ScrapedContent> {
        // Navigate to the page
        self.client.goto(url).await?;

        // Wait for page to load
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Execute actions
        for action in actions {
            match action {
                PageAction::Click { selector } => {
                    if let Ok(element) = self.client.find(Locator::Css(&selector)).await {
                        element.click().await?;
                        tokio::time::sleep(Duration::from_millis(500)).await;
                    }
                }
                PageAction::Type { selector, text } => {
                    if let Ok(element) = self.client.find(Locator::Css(&selector)).await {
                        element.clear().await?;
                        element.send_keys(&text).await?;
                        tokio::time::sleep(Duration::from_millis(200)).await;
                    }
                }
                PageAction::Wait { duration_ms } => {
                    tokio::time::sleep(Duration::from_millis(duration_ms)).await;
                }
                PageAction::ScrollTo { selector } => {
                    let script = format!(
                        "document.querySelector('{}').scrollIntoView();",
                        selector.replace("'", "\\'")
                    );
                    let _ = self.client.execute(&script, vec![]).await;
                    tokio::time::sleep(Duration::from_millis(300)).await;
                }
            }
        }

        // Extract final content
        let html = self.client.source().await?;
        let title = self.client.title().await.unwrap_or_default();
        let current_url = self.client.current_url().await?.to_string();

        Ok(ScrapedContent {
            url: current_url,
            title,
            html,
            screenshot: None,
            timestamp: chrono::Utc::now(),
        })
    }
}

impl Drop for BrowserInstance {
    fn drop(&mut self) {
        // Browser cleanup is handled by fantoccini automatically
        // when the client goes out of scope
    }
}

/// Actions that can be performed on a web page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PageAction {
    Click { selector: String },
    Type { selector: String, text: String },
    Wait { duration_ms: u64 },
    ScrollTo { selector: String },
}

/// Content extracted from a web page using browser automation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedContent {
    pub url: String,
    pub title: String,
    pub html: String,
    pub screenshot: Option<Vec<u8>>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_config_defaults() {
        let config = BrowserConfig::default();
        assert_eq!(config.max_instances, 5);
        assert!(config.headless);
        assert_eq!(config.window_size, (1920, 1080));
        assert!(!config.capabilities.is_null());
    }

    #[test]
    fn test_page_action_serialization() {
        let action = PageAction::Click {
            selector: "#button".to_string(),
        };

        // Test that actions can be serialized/deserialized
        let json = serde_json::to_string(&action).unwrap();
        let deserialized: PageAction = serde_json::from_str(&json).unwrap();

        match deserialized {
            PageAction::Click { selector } => assert_eq!(selector, "#button"),
            _ => panic!("Wrong action type"),
        }
    }

    #[tokio::test]
    async fn test_browser_pool_creation() {
        let config = BrowserConfig {
            max_instances: 2,
            ..Default::default()
        };

        let pool = BrowserPool::new(config);
        assert_eq!(pool.semaphore.available_permits(), 2);
    }
}
