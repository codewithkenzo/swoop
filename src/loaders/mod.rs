use std::path::Path;
use std::io::BufRead;
use csv::Reader;
use serde::{Deserialize, Serialize};
use validator::Validate;
use url::Url;
use crate::error::{Error, Result};

/// CSV bulk loader for processing large lists of URLs
// CSV loading and validation is implemented directly in this module

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UrlEntry {
    #[validate(url)]
    pub url: String,
    pub expected_data_type: Option<String>,
    pub priority: Option<String>,
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone)]
pub struct LoaderConfig {
    pub max_urls: usize,
    pub validate_urls: bool,
    pub deduplicate: bool,
    pub skip_invalid: bool,
}

impl Default for LoaderConfig {
    fn default() -> Self {
        Self {
            max_urls: 10000,
            validate_urls: true,
            deduplicate: true,
            skip_invalid: true,
        }
    }
}

#[derive(Debug)]
pub struct LoaderStats {
    pub total_processed: usize,
    pub valid_urls: usize,
    pub invalid_urls: usize,
    pub duplicates_removed: usize,
    pub processing_time_ms: u128,
}

pub struct BulkLoader {
    config: LoaderConfig,
    stats: LoaderStats,
}

impl BulkLoader {
    pub fn new(config: LoaderConfig) -> Self {
        Self {
            config,
            stats: LoaderStats {
                total_processed: 0,
                valid_urls: 0,
                invalid_urls: 0,
                duplicates_removed: 0,
                processing_time_ms: 0,
            },
        }
    }

    /// Load URLs from CSV file
    pub async fn load_from_csv<P: AsRef<Path>>(&mut self, file_path: P) -> Result<Vec<UrlEntry>> {
        let start_time = std::time::Instant::now();
        
        let file = std::fs::File::open(file_path)?;
        let mut reader = Reader::from_reader(file);
        let mut urls = Vec::new();
        let mut seen_urls = std::collections::HashSet::new();

        for result in reader.deserialize() {
            if urls.len() >= self.config.max_urls {
                break;
            }

            let record: UrlEntry = match result {
                Ok(r) => r,
                Err(_) => {
                    self.stats.invalid_urls += 1;
                    if self.config.skip_invalid {
                        continue;
                    } else {
                        return Err(Error::Parser("Invalid CSV record".to_string()));
                    }
                }
            };

            self.stats.total_processed += 1;

            // Validate URL if configured
            if self.config.validate_urls {
                if let Err(_) = record.validate() {
                    self.stats.invalid_urls += 1;
                    if self.config.skip_invalid {
                        continue;
                    } else {
                        return Err(Error::Parser(format!("Invalid URL: {}", record.url)));
                    }
                }

                // Additional URL parsing validation
                if Url::parse(&record.url).is_err() {
                    self.stats.invalid_urls += 1;
                    if self.config.skip_invalid {
                        continue;
                    } else {
                        return Err(Error::Parser(format!("Malformed URL: {}", record.url)));
                    }
                }
            }

            // Deduplicate if configured
            if self.config.deduplicate {
                if seen_urls.contains(&record.url) {
                    self.stats.duplicates_removed += 1;
                    continue;
                }
                seen_urls.insert(record.url.clone());
            }

            urls.push(record);
            self.stats.valid_urls += 1;
        }

        self.stats.processing_time_ms = start_time.elapsed().as_millis();
        Ok(urls)
    }

    /// Load URLs from plain text file (one URL per line)
    pub async fn load_from_text<P: AsRef<Path>>(&mut self, file_path: P) -> Result<Vec<UrlEntry>> {
        let start_time = std::time::Instant::now();
        
        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);
        let mut urls = Vec::new();
        let mut seen_urls = std::collections::HashSet::new();

        for (line_num, line) in reader.lines().enumerate() {
            if urls.len() >= self.config.max_urls {
                break;
            }

            let url_str = match line {
                Ok(l) => l.trim().to_string(),
                Err(_) => {
                    self.stats.invalid_urls += 1;
                    continue;
                }
            };

            if url_str.is_empty() || url_str.starts_with('#') {
                continue; // Skip empty lines and comments
            }

            self.stats.total_processed += 1;

            let entry = UrlEntry {
                url: url_str.clone(),
                expected_data_type: Some("html".to_string()),
                priority: Some("medium".to_string()),
                metadata: None,
            };

            // Validate URL if configured
            if self.config.validate_urls {
                if let Err(_) = entry.validate() {
                    self.stats.invalid_urls += 1;
                    if self.config.skip_invalid {
                        continue;
                    } else {
                        return Err(Error::Parser(format!("Invalid URL at line {}: {}", line_num + 1, url_str)));
                    }
                }

                if Url::parse(&url_str).is_err() {
                    self.stats.invalid_urls += 1;
                    if self.config.skip_invalid {
                        continue;
                    } else {
                        return Err(Error::Parser(format!("Malformed URL at line {}: {}", line_num + 1, url_str)));
                    }
                }
            }

            // Deduplicate if configured
            if self.config.deduplicate {
                if seen_urls.contains(&url_str) {
                    self.stats.duplicates_removed += 1;
                    continue;
                }
                seen_urls.insert(url_str);
            }

            urls.push(entry);
            self.stats.valid_urls += 1;
        }

        self.stats.processing_time_ms = start_time.elapsed().as_millis();
        Ok(urls)
    }

    /// Get loading statistics
    pub fn get_stats(&self) -> &LoaderStats {
        &self.stats
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = LoaderStats {
            total_processed: 0,
            valid_urls: 0,
            invalid_urls: 0,
            duplicates_removed: 0,
            processing_time_ms: 0,
        };
    }
} 