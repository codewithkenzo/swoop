use std::env;

#[derive(Debug, Clone)]
pub struct EnvConfig {
    pub database_url: String,
    pub storage_dir: String,
    pub llm_provider: String,
    pub llm_api_key: String,
    pub cors_origin: Option<String>,
    pub rust_log: String,
}

impl EnvConfig {
    pub fn load() -> Self {
        // Load from .env if present
        let _ = dotenvy::dotenv();

        Self {
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://swoop.db".into()),
            storage_dir: env::var("STORAGE_DIR").unwrap_or_else(|_| "swoop_data".into()),
            llm_provider: env::var("LLM_PROVIDER").unwrap_or_else(|_| "openai".into()),
            llm_api_key: env::var("LLM_API_KEY").unwrap_or_default(),
            cors_origin: env::var("CORS_ORIGIN").ok(),
            rust_log: env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        }
    }
} 