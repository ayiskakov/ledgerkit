use async_trait::async_trait;
use chrono::Utc;
use lk_core::connector::*;
use lk_core::webhook::WebhookVerifier;
use lk_types::currency::Currency;
use lk_types::event::{CanonicalEvent, EventKind};
use lk_types::money::Money;
use lk_types::payment::PaymentId;
use lk_types::provider::{ProviderCapability, ProviderId};
use lk_types::webhook::{RawWebhook, VerificationResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Configuration for the mock connector.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockConfig {
    /// Whether authorizations should succeed.
    pub authorize_success: bool,
    /// Whether captures should succeed.
    pub capture_success: bool,
    /// Whether refunds should succeed.
    pub refund_success: bool,
    /// Simulated latency in milliseconds.
    pub latency_ms: u64,
    /// Webhook signing secret for verification.
    pub webhook_secret: String,
}

impl Default for MockConfig {
    fn default() -> Self {
        Self {
            authorize_success: true,
            capture_success: true,
            refund_success: true,
            latency_ms: 0,
            webhook_secret: "mock_webhook_secret_key".to_string(),
        }
    }
}

/// In-memory record of a mock transaction.
#[derive(Debug, Clone)]
struct MockTransaction {
    payment_id: PaymentId,
    provider_ref: String,
    amount: Money,
    state: lk_types::payment::PaymentState,
}

/// Mock payment connector for testing and development.
///
/// Simulates a payment provider entirely in memory. Useful for:
/// - Integration testing without real provider credentials
/// - Local development and prototyping
/// - Fixture generation and webhook replay testing
pub struct MockConnector {
    provider_id: ProviderId,
    config: MockConfig,
    transactions: Arc<Mutex<HashMap<String, MockTransaction>>>,
}

impl MockConnector {
    pub fn new(config: MockConfig) -> Self {
        Self {
            provider_id: ProviderId::new("mock"),
            config,
            transactions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(MockConfig::default())
    }

    /// Get the current state of a mock transaction.
    pub fn get_transaction(&self, provider_ref: &str) -> Option<lk_types::payment::PaymentState> {
        self.transactions
            .lock()
            .unwrap()
            .get(provider_ref)
            .map(|t| t.state)
    }

    async fn simulate_latency(&self) {
        if self.config.latency_ms > 0 {
            tokio::time::sleep(std::time::Duration::from_millis(self.config.latency_ms)).await;
        }
    }
}

/// Error type for the mock connector.
#[derive(Debug, thiserror::Error)]
pub enum MockConnectorError {
    #[error("mock authorization declined")]
    AuthorizationDeclined,
    #[error("mock capture failed")]
    CaptureFailed,
    #[error("mock refund failed")]
    RefundFailed,
    #[error("transaction not found: {0}")]
    TransactionNotFound(String),
    #[error("invalid webhook payload: {0}")]
    InvalidWebhook(String),
}

#[async_trait]
impl lk_core::connector::PaymentConnector for MockConnector {
    type Error = MockConnectorError;

    fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    fn capabilities(&self) -> &[ProviderCapability] {
        &[
            ProviderCapability::AuthCapture,
            ProviderCapability::DirectCapture,
            ProviderCapability::Refund,
            ProviderCapability::PartialRefund,
            ProviderCapability::Void,
            ProviderCapability::Webhooks,
        ]
    }

    async fn authorize(&self, req: AuthorizeRequest) -> Result<AuthorizeResponse, Self::Error> {
        self.simulate_latency().await;

        if !self.config.authorize_success {
            return Err(MockConnectorError::AuthorizationDeclined);
        }

        let provider_ref = format!("mock_auth_{}", Uuid::new_v4());

        let txn = MockTransaction {
            payment_id: req.payment_id.clone(),
            provider_ref: provider_ref.clone(),
            amount: req.amount,
            state: lk_types::payment::PaymentState::Authorized,
        };

        self.transactions
            .lock()
            .unwrap()
            .insert(provider_ref.clone(), txn);

        Ok(AuthorizeResponse {
            payment_id: req.payment_id,
            provider_reference: provider_ref,
            authorized_amount: req.amount,
            raw_response: Some(serde_json::json!({
                "mock": true,
                "status": "authorized"
            })),
        })
    }

    async fn capture(&self, req: CaptureRequest) -> Result<CaptureResponse, Self::Error> {
        self.simulate_latency().await;

        if !self.config.capture_success {
            return Err(MockConnectorError::CaptureFailed);
        }

        let mut txns = self.transactions.lock().unwrap();
        let txn = txns
            .get_mut(&req.provider_reference)
            .ok_or_else(|| MockConnectorError::TransactionNotFound(req.provider_reference.clone()))?;

        let captured_amount = req.amount.unwrap_or(txn.amount);
        txn.state = lk_types::payment::PaymentState::Captured;

        Ok(CaptureResponse {
            payment_id: req.payment_id,
            provider_reference: req.provider_reference,
            captured_amount,
            raw_response: Some(serde_json::json!({
                "mock": true,
                "status": "captured"
            })),
        })
    }

    async fn refund(&self, req: RefundRequest) -> Result<RefundResponse, Self::Error> {
        self.simulate_latency().await;

        if !self.config.refund_success {
            return Err(MockConnectorError::RefundFailed);
        }

        let mut txns = self.transactions.lock().unwrap();
        let txn = txns
            .get_mut(&req.provider_reference)
            .ok_or_else(|| MockConnectorError::TransactionNotFound(req.provider_reference.clone()))?;

        let refunded_amount = req.amount.unwrap_or(txn.amount);
        txn.state = lk_types::payment::PaymentState::Refunded;
        let refund_ref = format!("mock_refund_{}", Uuid::new_v4());

        Ok(RefundResponse {
            payment_id: req.payment_id,
            provider_reference: req.provider_reference,
            refund_reference: refund_ref,
            refunded_amount,
            raw_response: Some(serde_json::json!({
                "mock": true,
                "status": "refunded"
            })),
        })
    }

    async fn parse_webhook(&self, webhook: RawWebhook) -> Result<CanonicalEvent, Self::Error> {
        let payload: serde_json::Value = serde_json::from_str(&webhook.body)
            .map_err(|e| MockConnectorError::InvalidWebhook(e.to_string()))?;

        let event_type = payload
            .get("type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| MockConnectorError::InvalidWebhook("missing 'type' field".to_string()))?;

        let kind = match event_type {
            "payment.authorized" => EventKind::PaymentAuthorized,
            "payment.captured" => EventKind::PaymentCaptured,
            "payment.failed" => EventKind::PaymentFailed,
            "payment.refunded" => EventKind::PaymentRefunded,
            other => {
                return Err(MockConnectorError::InvalidWebhook(format!(
                    "unknown event type: {}",
                    other
                )))
            }
        };

        let payment_id = payload
            .get("payment_id")
            .and_then(|v| v.as_str())
            .map(PaymentId::from_str);

        let amount = payload
            .get("amount")
            .and_then(|v| v.as_i64())
            .map(|a| Money::new(a, Currency::USD));

        let mut event = CanonicalEvent::new(kind)
            .with_provider(ProviderId::new("mock"))
            .with_raw_payload(payload);

        if let Some(pid) = payment_id {
            event = event.with_payment_id(pid);
        }
        if let Some(amt) = amount {
            event = event.with_amount(amt);
        }

        Ok(event)
    }
}

impl WebhookVerifier for MockConnector {
    type Error = MockConnectorError;

    fn verify(&self, webhook: &RawWebhook) -> Result<VerificationResult, Self::Error> {
        // Mock verifier: check for a simple "x-signature" header
        match webhook.header("x-signature") {
            Some(sig) if sig == self.config.webhook_secret => Ok(VerificationResult::Valid),
            Some(_) => Ok(VerificationResult::Invalid {
                reason: "signature mismatch".to_string(),
            }),
            None => Ok(VerificationResult::Skipped {
                reason: "no signature header present".to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lk_core::connector::PaymentConnector;
    use lk_types::currency::Currency;
    use lk_types::payment::PaymentMethod;

    #[tokio::test]
    async fn test_authorize_capture_refund() {
        let connector = MockConnector::with_defaults();

        // Authorize
        let auth_req = AuthorizeRequest {
            payment_id: PaymentId::new(),
            amount: Money::new(5000, Currency::USD),
            payment_method: PaymentMethod::Card,
            idempotency_key: None,
            metadata: serde_json::Value::Null,
        };
        let auth_resp = connector.authorize(auth_req).await.unwrap();
        assert_eq!(auth_resp.authorized_amount.amount, 5000);

        // Capture
        let cap_req = CaptureRequest {
            payment_id: auth_resp.payment_id.clone(),
            provider_reference: auth_resp.provider_reference.clone(),
            amount: None,
            idempotency_key: None,
        };
        let cap_resp = connector.capture(cap_req).await.unwrap();
        assert_eq!(cap_resp.captured_amount.amount, 5000);

        // Refund
        let ref_req = RefundRequest {
            payment_id: auth_resp.payment_id,
            provider_reference: auth_resp.provider_reference,
            amount: Some(Money::new(2000, Currency::USD)),
            reason: Some("customer request".to_string()),
            idempotency_key: None,
        };
        let ref_resp = connector.refund(ref_req).await.unwrap();
        assert_eq!(ref_resp.refunded_amount.amount, 2000);
    }

    #[tokio::test]
    async fn test_declined_authorization() {
        let config = MockConfig {
            authorize_success: false,
            ..Default::default()
        };
        let connector = MockConnector::new(config);

        let req = AuthorizeRequest {
            payment_id: PaymentId::new(),
            amount: Money::new(1000, Currency::EUR),
            payment_method: PaymentMethod::Card,
            idempotency_key: None,
            metadata: serde_json::Value::Null,
        };

        let result = connector.authorize(req).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_webhook_verification() {
        let connector = MockConnector::with_defaults();

        let mut headers = HashMap::new();
        headers.insert(
            "x-signature".to_string(),
            "mock_webhook_secret_key".to_string(),
        );

        let webhook = RawWebhook::new(
            headers,
            r#"{"type":"payment.captured","payment_id":"pay_123","amount":5000}"#.to_string(),
        );

        let result = connector.verify(&webhook).unwrap();
        assert!(result.is_valid());
    }
}
