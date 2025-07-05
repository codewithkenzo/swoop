# Security & Compliance Guide

## Overview

Swoop is designed with security-first principles, implementing enterprise-grade protection for your documents and data. This guide covers security architecture, compliance frameworks, and best practices.

## Security Architecture

### Threat Model

```
┌─────────────────────────────────────────────────────────────────┐
│                        Threat Vectors                           │
├─────────────────────────────────────────────────────────────────┤
│  External Threats          │  Internal Threats                  │
│  • API attacks             │  • Privilege escalation            │
│  • Data breaches           │  • Data exfiltration               │
│  • DDoS attacks            │  • Misconfigurations               │
│  • Injection attacks       │  • Insider threats                 │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Security Controls                            │
├─────────────────────────────────────────────────────────────────┤
│  Authentication & Authorization                                 │
│  • API key management                                           │
│  • Role-based access control                                    │
│  • Rate limiting                                                │
│  • Session management                                           │
│                                                                 │
│  Data Protection                                                │
│  • Encryption at rest (AES-256)                                │
│  • Encryption in transit (TLS 1.3)                             │
│  • Secure key management                                        │
│  • Data anonymization                                           │
│                                                                 │
│  Application Security                                           │
│  • Input validation & sanitization                             │
│  • SQL injection prevention                                     │
│  • XSS protection                                               │
│  • CSRF protection                                              │
│                                                                 │
│  Infrastructure Security                                        │
│  • Container security                                           │
│  • Network segmentation                                         │
│  • Monitoring & alerting                                        │
│  • Audit logging                                                │
└─────────────────────────────────────────────────────────────────┘
```

### Data Flow Security

```
┌─────────────┐    TLS 1.3     ┌─────────────┐    Encrypted    ┌─────────────┐
│   Client    │ ──────────────► │   API       │ ──────────────► │  Database   │
│             │                │  Gateway    │                │             │
└─────────────┘                └─────────────┘                └─────────────┘
      │                              │                              │
      │ API Key Auth                 │ Input Validation             │ AES-256
      │ Rate Limiting                │ SQL Injection Prevention     │ Encryption
      │ CORS Headers                 │ Request Sanitization         │ Access Control
      ▼                              ▼                              ▼
┌─────────────┐    Vector DB    ┌─────────────┐    External     ┌─────────────┐
│   Audit     │ ◄──────────────── │  Processing │ ──────────────► │  AI APIs    │
│   Logs      │                │   Engine    │                │             │
└─────────────┘                └─────────────┘                └─────────────┘
```

## Authentication & Authorization

### API Key Management

```rust
// src/auth/api_keys.rs
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use rand::Rng;

pub struct ApiKeyManager {
    hasher: Argon2<'static>,
    salt: [u8; 32],
}

impl ApiKeyManager {
    pub fn new() -> Self {
        Self {
            hasher: Argon2::default(),
            salt: std::env::var("API_KEY_SALT")
                .expect("API_KEY_SALT must be set")
                .as_bytes()
                .try_into()
                .expect("Invalid salt length"),
        }
    }
    
    pub fn generate_key(&self, user_id: &str) -> Result<String> {
        let random_bytes: [u8; 32] = rand::thread_rng().gen();
        let key = format!("swoop_sk_{}", hex::encode(random_bytes));
        let hash = self.hash_key(&key)?;
        
        // Store hash in database
        self.store_key_hash(user_id, &hash)?;
        
        Ok(key)
    }
    
    pub fn verify_key(&self, key: &str) -> Result<bool> {
        let stored_hash = self.get_stored_hash(key)?;
        match stored_hash {
            Some(hash) => Ok(self.hasher.verify_password(key.as_bytes(), &hash).is_ok()),
            None => Ok(false),
        }
    }
    
    fn hash_key(&self, key: &str) -> Result<String> {
        let hash = self.hasher.hash_password(key.as_bytes(), &self.salt)?;
        Ok(hash.to_string())
    }
}
```

### Role-Based Access Control

```rust
// src/auth/rbac.rs
#[derive(Debug, Clone, PartialEq)]
pub enum Permission {
    DocumentRead,
    DocumentWrite,
    DocumentDelete,
    AdminAccess,
    ApiKeyManage,
}

#[derive(Debug, Clone)]
pub enum Role {
    User,
    Admin,
    Service,
}

impl Role {
    pub fn permissions(&self) -> Vec<Permission> {
        match self {
            Role::User => vec![
                Permission::DocumentRead,
                Permission::DocumentWrite,
            ],
            Role::Admin => vec![
                Permission::DocumentRead,
                Permission::DocumentWrite,
                Permission::DocumentDelete,
                Permission::AdminAccess,
                Permission::ApiKeyManage,
            ],
            Role::Service => vec![
                Permission::DocumentRead,
                Permission::DocumentWrite,
            ],
        }
    }
    
    pub fn can(&self, permission: Permission) -> bool {
        self.permissions().contains(&permission)
    }
}
```

### Rate Limiting

```rust
// src/middleware/rate_limit.rs
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

pub struct RateLimiter {
    limits: Arc<RwLock<HashMap<String, RateLimit>>>,
    window: Duration,
    max_requests: u32,
}

#[derive(Debug)]
struct RateLimit {
    count: u32,
    window_start: Instant,
}

impl RateLimiter {
    pub fn new(window: Duration, max_requests: u32) -> Self {
        Self {
            limits: Arc::new(RwLock::new(HashMap::new())),
            window,
            max_requests,
        }
    }
    
    pub async fn check_rate_limit(&self, key: &str) -> bool {
        let mut limits = self.limits.write().await;
        let now = Instant::now();
        
        match limits.get_mut(key) {
            Some(limit) => {
                // Reset window if expired
                if now.duration_since(limit.window_start) > self.window {
                    limit.count = 1;
                    limit.window_start = now;
                    true
                } else if limit.count < self.max_requests {
                    limit.count += 1;
                    true
                } else {
                    false // Rate limit exceeded
                }
            }
            None => {
                limits.insert(key.to_string(), RateLimit {
                    count: 1,
                    window_start: now,
                });
                true
            }
        }
    }
}
```

## Data Protection

### Encryption at Rest

```rust
// src/security/encryption.rs
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use rand::Rng;

pub struct DataEncryption {
    cipher: Aes256Gcm,
}

impl DataEncryption {
    pub fn new() -> Result<Self> {
        let key_bytes = std::env::var("ENCRYPTION_KEY")
            .expect("ENCRYPTION_KEY must be set")
            .as_bytes()
            .try_into()
            .expect("Invalid key length");
        
        let key = Key::from_slice(key_bytes);
        let cipher = Aes256Gcm::new(key);
        
        Ok(Self { cipher })
    }
    
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let nonce_bytes: [u8; 12] = rand::thread_rng().gen();
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let mut encrypted = self.cipher.encrypt(nonce, data)
            .map_err(|e| Error::EncryptionFailed(e.to_string()))?;
        
        // Prepend nonce to encrypted data
        let mut result = nonce_bytes.to_vec();
        result.append(&mut encrypted);
        
        Ok(result)
    }
    
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < 12 {
            return Err(Error::InvalidData("Data too short".to_string()));
        }
        
        let (nonce_bytes, encrypted) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        self.cipher.decrypt(nonce, encrypted)
            .map_err(|e| Error::DecryptionFailed(e.to_string()))
    }
}
```

### Secure Configuration

```rust
// src/config/security.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub encryption_key: String,
    pub api_key_salt: String,
    pub jwt_secret: String,
    pub rate_limits: RateLimitConfig,
    pub cors: CorsConfig,
    pub tls: TlsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_limit: u32,
    pub window_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub max_age: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    pub cert_path: String,
    pub key_path: String,
    pub min_version: String,
}

impl SecurityConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            encryption_key: env::var("ENCRYPTION_KEY")?,
            api_key_salt: env::var("API_KEY_SALT")?,
            jwt_secret: env::var("JWT_SECRET")?,
            rate_limits: RateLimitConfig {
                requests_per_minute: env::var("RATE_LIMIT_RPM")?.parse()?,
                burst_limit: env::var("RATE_LIMIT_BURST")?.parse()?,
                window_size: env::var("RATE_LIMIT_WINDOW")?.parse()?,
            },
            cors: CorsConfig {
                allowed_origins: env::var("CORS_ORIGINS")?
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
                allowed_methods: vec![
                    "GET".to_string(),
                    "POST".to_string(),
                    "PUT".to_string(),
                    "DELETE".to_string(),
                ],
                allowed_headers: vec![
                    "Authorization".to_string(),
                    "Content-Type".to_string(),
                ],
                max_age: 3600,
            },
            tls: TlsConfig {
                cert_path: env::var("TLS_CERT_PATH")?,
                key_path: env::var("TLS_KEY_PATH")?,
                min_version: "1.3".to_string(),
            },
        })
    }
}
```

## Audit Logging

### Comprehensive Logging

```rust
// src/audit/logger.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<String>,
    pub api_key_id: Option<String>,
    pub event_type: EventType,
    pub resource: String,
    pub action: String,
    pub result: EventResult,
    pub metadata: serde_json::Value,
    pub client_ip: String,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Authentication,
    Authorization,
    DataAccess,
    DataModification,
    SystemChange,
    SecurityEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventResult {
    Success,
    Failure,
    Blocked,
}

pub struct AuditLogger {
    db: Arc<Database>,
}

impl AuditLogger {
    pub async fn log_event(&self, event: AuditEvent) -> Result<()> {
        // Store in database
        sqlx::query!(
            "INSERT INTO audit_logs (id, timestamp, user_id, api_key_id, event_type, resource, action, result, metadata, client_ip, user_agent) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
            event.id,
            event.timestamp,
            event.user_id,
            event.api_key_id,
            serde_json::to_string(&event.event_type)?,
            event.resource,
            event.action,
            serde_json::to_string(&event.result)?,
            event.metadata,
            event.client_ip,
            event.user_agent
        )
        .execute(&self.db.pool)
        .await?;
        
        // Also log to structured logging
        tracing::info!(
            event_id = %event.id,
            user_id = event.user_id,
            event_type = ?event.event_type,
            resource = event.resource,
            action = event.action,
            result = ?event.result,
            client_ip = event.client_ip,
            "Audit event logged"
        );
        
        Ok(())
    }
    
    pub async fn log_document_access(&self, user_id: &str, document_id: &str, action: &str, client_ip: &str) -> Result<()> {
        let event = AuditEvent {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            user_id: Some(user_id.to_string()),
            api_key_id: None,
            event_type: EventType::DataAccess,
            resource: format!("document:{}", document_id),
            action: action.to_string(),
            result: EventResult::Success,
            metadata: serde_json::json!({"document_id": document_id}),
            client_ip: client_ip.to_string(),
            user_agent: None,
        };
        
        self.log_event(event).await
    }
}
```

## Compliance Frameworks

### GDPR Compliance

**Data Subject Rights:**
- **Right to Access**: API endpoint for data export
- **Right to Rectification**: Document update capabilities
- **Right to Erasure**: Secure document deletion
- **Right to Portability**: JSON/PDF export formats
- **Right to Object**: Opt-out mechanisms

**Implementation:**
```rust
// src/compliance/gdpr.rs
pub struct GDPRCompliance {
    db: Arc<Database>,
    audit: Arc<AuditLogger>,
}

impl GDPRCompliance {
    pub async fn export_user_data(&self, user_id: &str) -> Result<UserDataExport> {
        let documents = self.get_user_documents(user_id).await?;
        let activities = self.get_user_activities(user_id).await?;
        let preferences = self.get_user_preferences(user_id).await?;
        
        let export = UserDataExport {
            user_id: user_id.to_string(),
            export_date: Utc::now(),
            documents,
            activities,
            preferences,
        };
        
        self.audit.log_event(AuditEvent {
            event_type: EventType::DataAccess,
            action: "gdpr_export".to_string(),
            // ... other fields
        }).await?;
        
        Ok(export)
    }
    
    pub async fn delete_user_data(&self, user_id: &str) -> Result<()> {
        // Secure deletion of all user data
        self.delete_user_documents(user_id).await?;
        self.delete_user_activities(user_id).await?;
        self.delete_user_preferences(user_id).await?;
        
        self.audit.log_event(AuditEvent {
            event_type: EventType::DataModification,
            action: "gdpr_deletion".to_string(),
            // ... other fields
        }).await?;
        
        Ok(())
    }
}
```

### SOC 2 Compliance

**Control Objectives:**
- **Security**: Encryption, access controls, monitoring
- **Availability**: High availability, disaster recovery
- **Processing Integrity**: Data validation, error handling
- **Confidentiality**: Data classification, access restrictions
- **Privacy**: Data handling, consent management

### HIPAA Compliance

**Safeguards:**
- **Administrative**: Security officer, workforce training
- **Physical**: Facility access, workstation controls
- **Technical**: Access control, audit controls, integrity, transmission security

## Security Best Practices

### Production Deployment

```yaml
# docker-compose.prod.yml
version: '3.8'
services:
  swoop-api:
    image: swoop:latest
    environment:
      # Security Configuration
      - ENCRYPTION_KEY=${ENCRYPTION_KEY}
      - API_KEY_SALT=${API_KEY_SALT}
      - JWT_SECRET=${JWT_SECRET}
      
      # Rate Limiting
      - RATE_LIMIT_RPM=100
      - RATE_LIMIT_BURST=20
      
      # CORS Configuration
      - CORS_ORIGINS=https://app.yourdomain.com,https://docs.yourdomain.com
      
      # TLS Configuration
      - TLS_CERT_PATH=/certs/server.crt
      - TLS_KEY_PATH=/certs/server.key
      
    volumes:
      - ./certs:/certs:ro
      - ./logs:/app/logs
    
    # Security options
    security_opt:
      - no-new-privileges:true
    
    # Run as non-root user
    user: "1000:1000"
    
    # Resource limits
    deploy:
      resources:
        limits:
          memory: 2G
          cpus: '1.0'
```

### Environment Variables

```bash
# .env.production
# Encryption Keys (32 bytes, base64 encoded)
ENCRYPTION_KEY="your-32-byte-encryption-key-base64"
API_KEY_SALT="your-32-byte-salt-base64"
JWT_SECRET="your-jwt-secret-key"

# Database Security
DATABASE_URL="postgresql://user:password@localhost:5432/swoop?sslmode=require"
DATABASE_SSL_CERT="/path/to/client-cert.pem"
DATABASE_SSL_KEY="/path/to/client-key.pem"
DATABASE_SSL_CA="/path/to/ca-cert.pem"

# External API Security
OPENROUTER_API_KEY="your-openrouter-key"
ELEVENLABS_API_KEY="your-elevenlabs-key"

# Network Security
CORS_ORIGINS="https://yourdomain.com,https://app.yourdomain.com"
ALLOWED_IPS="10.0.0.0/8,172.16.0.0/12,192.168.0.0/16"

# Logging
LOG_LEVEL="info"
AUDIT_LOG_RETENTION_DAYS="365"
```

### Security Checklist

**Infrastructure Security:**
- [ ] TLS 1.3 encryption for all communications
- [ ] Regular security updates and patches
- [ ] Network segmentation and firewalls
- [ ] Intrusion detection and prevention
- [ ] Regular security scanning and penetration testing

**Application Security:**
- [ ] Input validation and sanitization
- [ ] SQL injection prevention
- [ ] XSS and CSRF protection
- [ ] Secure session management
- [ ] Regular dependency updates

**Data Security:**
- [ ] Encryption at rest and in transit
- [ ] Secure key management
- [ ] Regular backups with encryption
- [ ] Data classification and handling
- [ ] Secure deletion procedures

**Access Control:**
- [ ] Strong authentication mechanisms
- [ ] Role-based access control
- [ ] Principle of least privilege
- [ ] Regular access reviews
- [ ] Multi-factor authentication

**Monitoring and Auditing:**
- [ ] Comprehensive audit logging
- [ ] Real-time security monitoring
- [ ] Incident response procedures
- [ ] Regular log analysis
- [ ] Compliance reporting

## Incident Response

### Security Incident Procedures

```rust
// src/security/incident.rs
#[derive(Debug, Clone)]
pub enum IncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct SecurityIncident {
    pub id: Uuid,
    pub severity: IncidentSeverity,
    pub description: String,
    pub affected_resources: Vec<String>,
    pub detected_at: DateTime<Utc>,
    pub reported_by: String,
    pub status: IncidentStatus,
}

#[derive(Debug, Clone)]
pub enum IncidentStatus {
    Detected,
    Investigating,
    Contained,
    Resolved,
}

pub struct IncidentResponse {
    audit: Arc<AuditLogger>,
    notification: Arc<NotificationService>,
}

impl IncidentResponse {
    pub async fn report_incident(&self, incident: SecurityIncident) -> Result<()> {
        // Log incident
        self.audit.log_event(AuditEvent {
            event_type: EventType::SecurityEvent,
            action: "incident_reported".to_string(),
            // ... other fields
        }).await?;
        
        // Notify security team
        match incident.severity {
            IncidentSeverity::Critical | IncidentSeverity::High => {
                self.notification.send_immediate_alert(&incident).await?;
            }
            _ => {
                self.notification.send_standard_alert(&incident).await?;
            }
        }
        
        Ok(())
    }
}
```

## Vulnerability Management

### Security Updates

- **Dependency Scanning**: Automated vulnerability scanning with `cargo audit`
- **Container Scanning**: Regular image scanning with Trivy/Snyk
- **Code Analysis**: Static analysis with Clippy and security linters
- **Penetration Testing**: Quarterly third-party security assessments

### Reporting Vulnerabilities

**Security Contact:**
- Email: security@swoop.dev
- PGP Key: [Public key available]
- Response Time: 24-48 hours

**Responsible Disclosure:**
1. Report vulnerability privately
2. Allow 90 days for fix development
3. Coordinate public disclosure
4. Recognition in security acknowledgments

---

This security guide provides comprehensive coverage of Swoop's security architecture, compliance frameworks, and best practices. Regular security reviews and updates ensure ongoing protection of your data and systems.