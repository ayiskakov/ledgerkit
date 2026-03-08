use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A stored idempotency record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdempotencyRecord {
    /// The idempotency key.
    pub key: String,
    /// When this record was created.
    pub created_at: DateTime<Utc>,
    /// When this record expires.
    pub expires_at: DateTime<Utc>,
    /// The stored response, if the operation completed.
    pub response: Option<serde_json::Value>,
    /// Current status of the operation.
    pub status: IdempotencyStatus,
}

/// Status of an idempotent operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdempotencyStatus {
    /// Operation is in progress.
    InProgress,
    /// Operation completed successfully.
    Completed,
    /// Operation failed.
    Failed,
}

/// Trait for idempotency key storage.
///
/// Provides deduplication for payment operations, ensuring that retried
/// requests don't result in duplicate charges.
#[async_trait]
pub trait IdempotencyStore: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Try to acquire an idempotency lock for the given key.
    ///
    /// Returns `Ok(None)` if the key is new (lock acquired).
    /// Returns `Ok(Some(record))` if the key already exists.
    async fn try_acquire(&self, key: &str, ttl_secs: u64) -> Result<Option<IdempotencyRecord>, Self::Error>;

    /// Mark an idempotent operation as completed with a response.
    async fn complete(&self, key: &str, response: serde_json::Value) -> Result<(), Self::Error>;

    /// Mark an idempotent operation as failed.
    async fn fail(&self, key: &str) -> Result<(), Self::Error>;

    /// Get an existing idempotency record.
    async fn get(&self, key: &str) -> Result<Option<IdempotencyRecord>, Self::Error>;

    /// Remove an idempotency record.
    async fn remove(&self, key: &str) -> Result<(), Self::Error>;
}
