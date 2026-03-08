use async_trait::async_trait;
use lk_types::event::CanonicalEvent;
use lk_types::money::Money;
use lk_types::payment::{PaymentId, PaymentMethod};
use lk_types::provider::{ProviderCapability, ProviderId};
use lk_types::webhook::RawWebhook;
use serde::{Deserialize, Serialize};

/// Request to authorize a payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizeRequest {
    pub payment_id: PaymentId,
    pub amount: Money,
    pub payment_method: PaymentMethod,
    pub idempotency_key: Option<String>,
    pub metadata: serde_json::Value,
}

/// Response from an authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizeResponse {
    pub payment_id: PaymentId,
    pub provider_reference: String,
    pub authorized_amount: Money,
    pub raw_response: Option<serde_json::Value>,
}

/// Request to capture an authorized payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureRequest {
    pub payment_id: PaymentId,
    pub provider_reference: String,
    pub amount: Option<Money>,
    pub idempotency_key: Option<String>,
}

/// Response from a capture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureResponse {
    pub payment_id: PaymentId,
    pub provider_reference: String,
    pub captured_amount: Money,
    pub raw_response: Option<serde_json::Value>,
}

/// Request to refund a payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundRequest {
    pub payment_id: PaymentId,
    pub provider_reference: String,
    pub amount: Option<Money>,
    pub reason: Option<String>,
    pub idempotency_key: Option<String>,
}

/// Response from a refund.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
    pub payment_id: PaymentId,
    pub provider_reference: String,
    pub refund_reference: String,
    pub refunded_amount: Money,
    pub raw_response: Option<serde_json::Value>,
}

/// The core trait for payment provider connectors.
///
/// Every PSP/acquirer integration implements this trait, providing a
/// consistent interface for authorization, capture, refund, and webhook parsing.
#[async_trait]
pub trait PaymentConnector: Send + Sync {
    /// The error type for this connector.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Returns the provider identifier.
    fn provider_id(&self) -> &ProviderId;

    /// Returns the capabilities supported by this provider.
    fn capabilities(&self) -> &[ProviderCapability];

    /// Authorize a payment (reserve funds).
    async fn authorize(&self, req: AuthorizeRequest) -> Result<AuthorizeResponse, Self::Error>;

    /// Capture an authorized payment (transfer funds).
    async fn capture(&self, req: CaptureRequest) -> Result<CaptureResponse, Self::Error>;

    /// Refund a captured payment.
    async fn refund(&self, req: RefundRequest) -> Result<RefundResponse, Self::Error>;

    /// Parse a raw webhook into a canonical event.
    async fn parse_webhook(&self, webhook: RawWebhook) -> Result<CanonicalEvent, Self::Error>;
}
