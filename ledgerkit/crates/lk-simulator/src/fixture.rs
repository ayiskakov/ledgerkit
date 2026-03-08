use lk_types::currency::Currency;
use lk_types::event::{CanonicalEvent, EventKind};
use lk_types::money::Money;
use lk_types::payment::PaymentId;
use lk_types::provider::ProviderId;
use serde::{Deserialize, Serialize};

/// A test fixture describing a sequence of payment events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fixture {
    pub name: String,
    pub description: Option<String>,
    pub provider: String,
    pub events: Vec<FixtureEvent>,
}

/// A single event in a fixture sequence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixtureEvent {
    pub kind: EventKind,
    pub amount_minor: i64,
    pub currency: Currency,
    pub delay_ms: Option<u64>,
    pub metadata: Option<serde_json::Value>,
}

impl Fixture {
    /// Generate a standard successful payment fixture.
    pub fn successful_payment(amount: i64, currency: Currency) -> Self {
        Self {
            name: "successful_payment".to_string(),
            description: Some("A standard authorize -> capture flow".to_string()),
            provider: "mock".to_string(),
            events: vec![
                FixtureEvent {
                    kind: EventKind::PaymentCreated,
                    amount_minor: amount,
                    currency,
                    delay_ms: None,
                    metadata: None,
                },
                FixtureEvent {
                    kind: EventKind::PaymentAuthorized,
                    amount_minor: amount,
                    currency,
                    delay_ms: Some(100),
                    metadata: None,
                },
                FixtureEvent {
                    kind: EventKind::PaymentCaptured,
                    amount_minor: amount,
                    currency,
                    delay_ms: Some(200),
                    metadata: None,
                },
            ],
        }
    }

    /// Generate a failed payment fixture.
    pub fn failed_payment(amount: i64, currency: Currency) -> Self {
        Self {
            name: "failed_payment".to_string(),
            description: Some("A payment that fails during authorization".to_string()),
            provider: "mock".to_string(),
            events: vec![
                FixtureEvent {
                    kind: EventKind::PaymentCreated,
                    amount_minor: amount,
                    currency,
                    delay_ms: None,
                    metadata: None,
                },
                FixtureEvent {
                    kind: EventKind::PaymentFailed,
                    amount_minor: amount,
                    currency,
                    delay_ms: Some(100),
                    metadata: Some(serde_json::json!({"reason": "insufficient_funds"})),
                },
            ],
        }
    }

    /// Generate a refund fixture.
    pub fn refunded_payment(amount: i64, currency: Currency) -> Self {
        Self {
            name: "refunded_payment".to_string(),
            description: Some("A captured payment that is fully refunded".to_string()),
            provider: "mock".to_string(),
            events: vec![
                FixtureEvent {
                    kind: EventKind::PaymentCreated,
                    amount_minor: amount,
                    currency,
                    delay_ms: None,
                    metadata: None,
                },
                FixtureEvent {
                    kind: EventKind::PaymentAuthorized,
                    amount_minor: amount,
                    currency,
                    delay_ms: Some(100),
                    metadata: None,
                },
                FixtureEvent {
                    kind: EventKind::PaymentCaptured,
                    amount_minor: amount,
                    currency,
                    delay_ms: Some(200),
                    metadata: None,
                },
                FixtureEvent {
                    kind: EventKind::PaymentRefunded,
                    amount_minor: amount,
                    currency,
                    delay_ms: Some(500),
                    metadata: Some(serde_json::json!({"reason": "customer_request"})),
                },
            ],
        }
    }

    /// Convert fixture events to canonical events.
    pub fn to_canonical_events(&self) -> Vec<CanonicalEvent> {
        let payment_id = PaymentId::new();
        let provider_id = ProviderId::new(&self.provider);

        self.events
            .iter()
            .map(|fe| {
                CanonicalEvent::new(fe.kind.clone())
                    .with_payment_id(payment_id.clone())
                    .with_provider(provider_id.clone())
                    .with_amount(Money::new(fe.amount_minor, fe.currency))
            })
            .collect()
    }
}
