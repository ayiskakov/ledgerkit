use async_trait::async_trait;
use chrono::Utc;
use lk_core::idempotency::{IdempotencyRecord, IdempotencyStatus, IdempotencyStore};
use lk_types::event::CanonicalEvent;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// In-memory idempotency store for development and testing.
pub struct InMemoryIdempotencyStore {
    records: Arc<Mutex<HashMap<String, IdempotencyRecord>>>,
}

impl InMemoryIdempotencyStore {
    pub fn new() -> Self {
        Self {
            records: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryIdempotencyStore {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("in-memory store error: {0}")]
pub struct MemoryStoreError(pub String);

#[async_trait]
impl IdempotencyStore for InMemoryIdempotencyStore {
    type Error = MemoryStoreError;

    async fn try_acquire(
        &self,
        key: &str,
        ttl_secs: u64,
    ) -> Result<Option<IdempotencyRecord>, Self::Error> {
        let mut records = self.records.lock().unwrap();

        if let Some(existing) = records.get(key) {
            if existing.expires_at < Utc::now() {
                records.remove(key);
            } else {
                return Ok(Some(existing.clone()));
            }
        }

        let now = Utc::now();
        let record = IdempotencyRecord {
            key: key.to_string(),
            created_at: now,
            expires_at: now + chrono::Duration::seconds(ttl_secs as i64),
            response: None,
            status: IdempotencyStatus::InProgress,
        };
        records.insert(key.to_string(), record);
        Ok(None)
    }

    async fn complete(&self, key: &str, response: serde_json::Value) -> Result<(), Self::Error> {
        let mut records = self.records.lock().unwrap();
        if let Some(record) = records.get_mut(key) {
            record.status = IdempotencyStatus::Completed;
            record.response = Some(response);
        }
        Ok(())
    }

    async fn fail(&self, key: &str) -> Result<(), Self::Error> {
        let mut records = self.records.lock().unwrap();
        if let Some(record) = records.get_mut(key) {
            record.status = IdempotencyStatus::Failed;
        }
        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<IdempotencyRecord>, Self::Error> {
        let records = self.records.lock().unwrap();
        Ok(records.get(key).cloned())
    }

    async fn remove(&self, key: &str) -> Result<(), Self::Error> {
        let mut records = self.records.lock().unwrap();
        records.remove(key);
        Ok(())
    }
}

/// In-memory event store for development and testing.
pub struct InMemoryEventStore {
    events: Arc<Mutex<Vec<CanonicalEvent>>>,
}

impl InMemoryEventStore {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn store(&self, event: CanonicalEvent) {
        self.events.lock().unwrap().push(event);
    }

    pub fn all(&self) -> Vec<CanonicalEvent> {
        self.events.lock().unwrap().clone()
    }

    pub fn count(&self) -> usize {
        self.events.lock().unwrap().len()
    }

    pub fn clear(&self) {
        self.events.lock().unwrap().clear();
    }
}

impl Default for InMemoryEventStore {
    fn default() -> Self {
        Self::new()
    }
}
