use crate::money::Money;
use crate::payment::{PaymentId, PaymentState};
use crate::provider::ProviderId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Canonical event types for the payment lifecycle.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    // Payment events
    PaymentCreated,
    PaymentProcessing,
    PaymentAuthorized,
    PaymentCaptured,
    PaymentFailed,
    PaymentCancelled,
    PaymentRefunded,
    PaymentPartiallyRefunded,
    PaymentExpired,

    // Payout events
    PayoutCreated,
    PayoutSent,
    PayoutFailed,
    PayoutCompleted,

    // Dispute events
    ChargebackOpened,
    ChargebackWon,
    ChargebackLost,
    ChargebackClosed,

    // Webhook events
    WebhookReceived,
    WebhookVerified,
    WebhookRejected,

    // System events
    ConnectorError,
    RetryScheduled,
    RetryExhausted,
}

impl EventKind {
    /// Returns the corresponding payment state, if applicable.
    pub fn to_payment_state(&self) -> Option<PaymentState> {
        match self {
            EventKind::PaymentCreated => Some(PaymentState::Created),
            EventKind::PaymentProcessing => Some(PaymentState::Processing),
            EventKind::PaymentAuthorized => Some(PaymentState::Authorized),
            EventKind::PaymentCaptured => Some(PaymentState::Captured),
            EventKind::PaymentFailed => Some(PaymentState::Failed),
            EventKind::PaymentCancelled => Some(PaymentState::Cancelled),
            EventKind::PaymentRefunded => Some(PaymentState::Refunded),
            EventKind::PaymentPartiallyRefunded => Some(PaymentState::PartiallyRefunded),
            EventKind::PaymentExpired => Some(PaymentState::Expired),
            _ => None,
        }
    }
}

impl std::fmt::Display for EventKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self)
            .ok()
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_else(|| format!("{:?}", self));
        write!(f, "{}", s)
    }
}

/// A canonical payment event that normalizes data from any provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalEvent {
    /// Unique event ID
    pub event_id: String,
    /// Type of event
    pub kind: EventKind,
    /// Associated payment ID
    pub payment_id: Option<PaymentId>,
    /// Provider that originated the event
    pub provider_id: Option<ProviderId>,
    /// Provider's reference for this event
    pub provider_event_id: Option<String>,
    /// Amount associated with the event (e.g., refund amount)
    pub amount: Option<Money>,
    /// When the event occurred at the provider
    pub occurred_at: DateTime<Utc>,
    /// When we received/processed the event
    pub received_at: DateTime<Utc>,
    /// Correlation ID for tracing
    pub correlation_id: String,
    /// Additional structured data
    pub metadata: serde_json::Value,
    /// Raw provider payload (for debugging)
    pub raw_payload: Option<serde_json::Value>,
}

impl CanonicalEvent {
    /// Create a new event with generated IDs.
    pub fn new(kind: EventKind) -> Self {
        let now = Utc::now();
        Self {
            event_id: Uuid::new_v4().to_string(),
            kind,
            payment_id: None,
            provider_id: None,
            provider_event_id: None,
            amount: None,
            occurred_at: now,
            received_at: now,
            correlation_id: Uuid::new_v4().to_string(),
            metadata: serde_json::Value::Null,
            raw_payload: None,
        }
    }

    /// Builder: set payment ID.
    pub fn with_payment_id(mut self, id: PaymentId) -> Self {
        self.payment_id = Some(id);
        self
    }

    /// Builder: set provider ID.
    pub fn with_provider(mut self, id: ProviderId) -> Self {
        self.provider_id = Some(id);
        self
    }

    /// Builder: set amount.
    pub fn with_amount(mut self, amount: Money) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Builder: set correlation ID.
    pub fn with_correlation_id(mut self, id: String) -> Self {
        self.correlation_id = id;
        self
    }

    /// Builder: set raw payload.
    pub fn with_raw_payload(mut self, payload: serde_json::Value) -> Self {
        self.raw_payload = Some(payload);
        self
    }
}
