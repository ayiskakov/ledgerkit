use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique webhook delivery ID.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WebhookId(pub String);

impl WebhookId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn from_str(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Default for WebhookId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for WebhookId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Raw incoming webhook before verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawWebhook {
    /// Webhook delivery ID (ours, not the provider's)
    pub id: WebhookId,
    /// HTTP headers from the webhook request
    pub headers: HashMap<String, String>,
    /// Raw body bytes as a string
    pub body: String,
    /// When the webhook was received
    pub received_at: DateTime<Utc>,
    /// Source IP address if available
    pub source_ip: Option<String>,
}

impl RawWebhook {
    pub fn new(headers: HashMap<String, String>, body: String) -> Self {
        Self {
            id: WebhookId::new(),
            headers,
            body,
            received_at: Utc::now(),
            source_ip: None,
        }
    }

    /// Get a header value (case-insensitive lookup).
    pub fn header(&self, name: &str) -> Option<&str> {
        let lower = name.to_lowercase();
        self.headers
            .iter()
            .find(|(k, _)| k.to_lowercase() == lower)
            .map(|(_, v)| v.as_str())
    }
}

/// Result of webhook signature verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationResult {
    /// Signature is valid
    Valid,
    /// Signature is invalid
    Invalid { reason: String },
    /// Timestamp is outside acceptable window
    TimestampExpired { received: String, tolerance_secs: u64 },
    /// Duplicate delivery detected
    Duplicate { original_id: String },
    /// Verification was skipped (e.g., in dev mode)
    Skipped { reason: String },
}

impl VerificationResult {
    pub fn is_valid(&self) -> bool {
        matches!(self, VerificationResult::Valid)
    }

    pub fn is_rejected(&self) -> bool {
        matches!(
            self,
            VerificationResult::Invalid { .. } | VerificationResult::TimestampExpired { .. }
        )
    }
}
