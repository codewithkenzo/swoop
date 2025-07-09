use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use hyper::Uri;

#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Invalid URL scheme '{scheme}'. Only http and https are allowed")]
    InvalidScheme { scheme: String },

    #[error("Access to private IP address '{ip}' is forbidden for security reasons")] 
    PrivateIP { ip: String },

    #[error("Access to domain '{domain}' is blocked for security reasons")]
    BlockedDomain { domain: String },

    #[error("URL validation failed: {reason}")]
    ValidationFailed { reason: String },

    #[error("Invalid port number: {port}")]
    InvalidPort { port: u16 },

    #[error("Invalid URL format: {details}")]
    MalformedUrl { details: String },
}

pub struct UrlValidator {
    allowed_schemes: Vec<String>,
    blocked_domains: Vec<String>,
    allow_private_ips: bool,
}

impl Default for UrlValidator {
    fn default() -> Self {
        Self {
            allowed_schemes: vec!["http".to_string(), "https".to_string()],
            blocked_domains: vec![
                "localhost".to_string(),
                "127.0.0.1".to_string(),
                "0.0.0.0".to_string(),
                "169.254.169.254".to_string(), // AWS metadata
            ],
            allow_private_ips: false,
        }
    }
}

impl UrlValidator {
    pub fn new(allow_private_ips: bool) -> Self {
        Self {
            allow_private_ips,
            ..Default::default()
        }
    }

    pub fn validate_url(&self, url: &str) -> Result<Uri, SecurityError> {
        let uri: Uri = url.parse().map_err(|e| SecurityError::ValidationFailed {
            reason: format!("Parse error: {}", e),
        })?;

        // Validate scheme
        let scheme = uri.scheme_str().unwrap_or("");
        if !self.allowed_schemes.contains(&scheme.to_string()) {
            return Err(SecurityError::InvalidScheme {
                scheme: scheme.to_string(),
            });
        }

        // Validate host
        if let Some(host) = uri.host() {
            // Check blocked domains
            if self.blocked_domains.iter().any(|blocked| host.contains(blocked)) {
                return Err(SecurityError::BlockedDomain {
                    domain: host.to_string(),
                });
            }

            // Check private IP ranges
            if !self.allow_private_ips && self.is_private_ip(host)? {
                return Err(SecurityError::PrivateIP {
                    ip: host.to_string(),
                });
            }
        }

        Ok(uri)
    }

    fn is_private_ip(&self, host: &str) -> Result<bool, SecurityError> {
        // Try to parse as IP address
        if let Ok(ip) = host.parse::<IpAddr>() {
            match ip {
                IpAddr::V4(ipv4) => Ok(self.is_private_ipv4(ipv4)),
                IpAddr::V6(ipv6) => Ok(self.is_private_ipv6(ipv6)),
            }
        } else {
            // If not an IP, assume it's a domain name and not a private IP
            Ok(false)
        }
    }

    fn is_private_ipv4(&self, ip: Ipv4Addr) -> bool {
        ip.is_private() 
            || ip.is_loopback() 
            || ip.is_link_local()
            || ip.is_broadcast()
            || ip.is_documentation()
            || ip.is_multicast()
            // Additional AWS/GCP metadata checks
            || ip.octets() == [169, 254, 169, 254] // AWS metadata
    }

    fn is_private_ipv6(&self, ip: Ipv6Addr) -> bool {
        ip.is_loopback() || ip.is_multicast() || ip.is_unspecified()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_https_url() {
        let validator = UrlValidator::default();
        let result = validator.validate_url("https://example.com");
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_scheme() {
        let validator = UrlValidator::default();
        let result = validator.validate_url("ftp://example.com");
        assert!(matches!(result, Err(SecurityError::InvalidScheme { .. })));
    }

    #[test]
    fn test_blocked_domain() {
        let validator = UrlValidator::default();
        let result = validator.validate_url("https://localhost:8080");
        assert!(matches!(result, Err(SecurityError::BlockedDomain { .. })));
    }

    #[test]
    fn test_private_ip_blocked() {
        let validator = UrlValidator::default();
        let result = validator.validate_url("https://192.168.1.1");
        assert!(matches!(result, Err(SecurityError::PrivateIP { .. })));
    }

    #[test]
    fn test_aws_metadata_blocked() {
        let validator = UrlValidator::default();
        let result = validator.validate_url("http://169.254.169.254/latest/meta-data/");
        assert!(matches!(result, Err(SecurityError::BlockedDomain { .. })));
    }

    #[test]
    fn test_allow_private_ips() {
        let validator = UrlValidator::new(true);
        let result = validator.validate_url("https://192.168.1.1");
        assert!(result.is_ok());
    }
}