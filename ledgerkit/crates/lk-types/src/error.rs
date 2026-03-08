use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Top-level error type for ledgerkit operations.
#[derive(Debug, Error)]
pub enum LkError {
    #[error("provider error: {category} - {message}")]
    Provider {
        category: ErrorCategory,
        message: String,
        provider_code: Option<String>,
        retryable: bool,
    },

    #[error("webhook error: {0}")]
    Webhook(String),

    #[error("invalid state transition from {from:?} to {to:?}")]
    InvalidStateTransition {
        from: crate::payment::PaymentState,
        to: crate::payment::PaymentState,
    },

    #[error("validation error: {0}")]
    Validation(String),

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("configuration error: {0}")]
    Configuration(String),

    #[error("timeout after {duration_ms}ms")]
    Timeout { duration_ms: u64 },

    #[error("not found: {entity} {id}")]
    NotFound { entity: String, id: String },

    #[error("idempotency conflict: key={key}")]
    IdempotencyConflict { key: String },

    #[error("internal error: {0}")]
    Internal(String),
}

impl LkError {
    /// Returns true if this error is retryable.
    pub fn is_retryable(&self) -> bool {
        match self {
            LkError::Provider { retryable, .. } => *retryable,
            LkError::Timeout { .. } => true,
            LkError::Internal(_) => true,
            _ => false,
        }
    }

    /// Create a provider error.
    pub fn provider(category: ErrorCategory, message: impl Into<String>) -> Self {
        let retryable = category.is_retryable();
        LkError::Provider {
            category,
            message: message.into(),
            provider_code: None,
            retryable,
        }
    }
}

/// Categories of provider/payment errors for consistent handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCategory {
    /// Authentication/authorization failure with provider
    Authentication,
    /// Request validation failure at provider
    InvalidRequest,
    /// Card/payment method declined
    Declined,
    /// Insufficient funds
    InsufficientFunds,
    /// Card expired
    CardExpired,
    /// Fraud check failed
    FraudSuspected,
    /// Provider rate limit exceeded
    RateLimited,
    /// Provider service unavailable
    ProviderUnavailable,
    /// Network/connection error
    NetworkError,
    /// Provider returned unexpected response
    UnexpectedResponse,
    /// Operation timed out
    Timeout,
    /// Idempotency conflict
    IdempotencyConflict,
    /// Generic processing error
    ProcessingError,
    /// Unknown/unmapped error
    Unknown,
}

impl ErrorCategory {
    /// Whether errors of this category are generally retryable.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            ErrorCategory::RateLimited
                | ErrorCategory::ProviderUnavailable
                | ErrorCategory::NetworkError
                | ErrorCategory::Timeout
        )
    }
}

impl std::fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self)
            .ok()
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| format!("{:?}", self));
        write!(f, "{}", s)
    }
}
