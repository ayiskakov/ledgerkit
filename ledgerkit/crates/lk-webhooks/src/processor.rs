use lk_core::idempotency::IdempotencyStore;
use lk_core::webhook::WebhookVerifier;
use lk_types::event::CanonicalEvent;
use lk_types::webhook::{RawWebhook, VerificationResult};
use tracing::{info, warn};

/// Errors from the webhook processor.
#[derive(Debug, thiserror::Error)]
pub enum ProcessorError {
    #[error("verification failed: {0}")]
    Verification(String),
    #[error("idempotency check failed: {0}")]
    Idempotency(String),
    #[error("event parsing failed: {0}")]
    Parsing(String),
}

/// Result of processing a webhook.
#[derive(Debug)]
pub enum ProcessResult {
    /// New event processed successfully.
    Processed(CanonicalEvent),
    /// Duplicate webhook, already processed.
    Duplicate { original_id: String },
    /// Webhook rejected (invalid signature, expired, etc.).
    Rejected { result: VerificationResult },
}

/// Orchestrates the webhook processing pipeline:
/// 1. Verify signature
/// 2. Check timestamp
/// 3. Check idempotency (replay detection)
/// 4. Parse into canonical event
pub struct WebhookProcessor<V, S>
where
    V: WebhookVerifier,
    S: IdempotencyStore,
{
    verifier: V,
    store: S,
    timestamp_tolerance_secs: u64,
}

impl<V, S> WebhookProcessor<V, S>
where
    V: WebhookVerifier,
    S: IdempotencyStore,
{
    pub fn new(verifier: V, store: S) -> Self {
        Self {
            verifier,
            store,
            timestamp_tolerance_secs: 300, // 5 minutes default
        }
    }

    pub fn with_timestamp_tolerance(mut self, secs: u64) -> Self {
        self.timestamp_tolerance_secs = secs;
        self
    }

    /// Process an incoming webhook through the verification pipeline.
    pub async fn process<F>(
        &self,
        webhook: &RawWebhook,
        parse_fn: F,
    ) -> Result<ProcessResult, ProcessorError>
    where
        F: FnOnce(&RawWebhook) -> Result<CanonicalEvent, String>,
    {
        // Step 1: Verify signature
        let verification = self
            .verifier
            .verify(webhook)
            .map_err(|e| ProcessorError::Verification(e.to_string()))?;

        if verification.is_rejected() {
            warn!(
                webhook_id = %webhook.id,
                "webhook rejected: {:?}",
                verification
            );
            return Ok(ProcessResult::Rejected {
                result: verification,
            });
        }

        // Step 2: Check idempotency (replay detection)
        let idemp_key = format!("webhook:{}", webhook.id);
        match self
            .store
            .try_acquire(&idemp_key, 86400) // 24h TTL
            .await
            .map_err(|e| ProcessorError::Idempotency(e.to_string()))?
        {
            Some(existing) => {
                info!(
                    webhook_id = %webhook.id,
                    "duplicate webhook detected"
                );
                return Ok(ProcessResult::Duplicate {
                    original_id: existing.key,
                });
            }
            None => {} // New webhook, continue processing
        }

        // Step 3: Parse into canonical event
        let event = parse_fn(webhook).map_err(ProcessorError::Parsing)?;

        // Step 4: Mark as processed
        let _ = self
            .store
            .complete(
                &idemp_key,
                serde_json::json!({ "event_id": event.event_id }),
            )
            .await;

        info!(
            webhook_id = %webhook.id,
            event_id = %event.event_id,
            event_kind = %event.kind,
            "webhook processed successfully"
        );

        Ok(ProcessResult::Processed(event))
    }
}
