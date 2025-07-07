use serde::{Deserialize, Serialize};
use std::env;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct SecureS3Config {
    pub endpoint: String,
    pub bucket: String,
    pub region: String,
    // Credentials loaded from environment - not stored in struct
}

impl SecureS3Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            endpoint: env::var("S3_ENDPOINT")
                .unwrap_or_else(|_| "https://s3.amazonaws.com".to_string()),
            bucket: env::var("S3_BUCKET")
                .map_err(|_| anyhow::anyhow!("S3_BUCKET environment variable required"))?,
            region: env::var("S3_REGION")
                .unwrap_or_else(|_| "us-east-1".to_string()),
        })
    }
    
    pub fn get_credentials() -> Result<(String, String)> {
        let access_key = env::var("AWS_ACCESS_KEY_ID")
            .map_err(|_| anyhow::anyhow!("AWS_ACCESS_KEY_ID environment variable required"))?;
        let secret_key = env::var("AWS_SECRET_ACCESS_KEY")
            .map_err(|_| anyhow::anyhow!("AWS_SECRET_ACCESS_KEY environment variable required"))?;
        
        if access_key.is_empty() || secret_key.is_empty() {
            return Err(anyhow::anyhow!("AWS credentials cannot be empty"));
        }
        
        Ok((access_key, secret_key))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureScyllaConfig {
    pub nodes: Vec<String>,
    pub keyspace: String,
    pub timeout_secs: u64,
}

impl Default for SecureScyllaConfig {
    fn default() -> Self {
        Self {
            nodes: vec!["127.0.0.1:9042".to_string()],
            keyspace: "swoop".to_string(),
            timeout_secs: 30,
        }
    }
}

impl SecureScyllaConfig {
    pub fn from_env() -> Result<Self> {
        let nodes_str = env::var("SCYLLA_NODES")
            .unwrap_or_else(|_| "127.0.0.1:9042".to_string());
        let nodes = nodes_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        Ok(Self {
            nodes,
            keyspace: env::var("SCYLLA_KEYSPACE")
                .unwrap_or_else(|_| "swoop".to_string()),
            timeout_secs: env::var("SCYLLA_TIMEOUT_SECS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_s3_config_missing_bucket() {
        env::remove_var("S3_BUCKET");
        let result = SecureS3Config::from_env();
        assert!(result.is_err());
    }

    #[test]
    fn test_s3_config_from_env() {
        env::set_var("S3_BUCKET", "test-bucket");
        env::set_var("S3_REGION", "us-west-2");
        
        let config = SecureS3Config::from_env().unwrap();
        assert_eq!(config.bucket, "test-bucket");
        assert_eq!(config.region, "us-west-2");
        
        // Cleanup
        env::remove_var("S3_BUCKET");
        env::remove_var("S3_REGION");
    }

    #[test]
    fn test_scylla_config_from_env() {
        env::set_var("SCYLLA_NODES", "node1:9042,node2:9042");
        env::set_var("SCYLLA_KEYSPACE", "test_keyspace");
        
        let config = SecureScyllaConfig::from_env().unwrap();
        assert_eq!(config.nodes, vec!["node1:9042", "node2:9042"]);
        assert_eq!(config.keyspace, "test_keyspace");
        
        // Cleanup
        env::remove_var("SCYLLA_NODES");
        env::remove_var("SCYLLA_KEYSPACE");
    }
}