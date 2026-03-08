use crate::money::Money;
use crate::provider::ProviderId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique payment identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PaymentId(pub String);

impl PaymentId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn from_str(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Default for PaymentId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for PaymentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// High-level payment state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentState {
    /// Payment has been created but not yet submitted.
    Created,
    /// Payment is being processed.
    Processing,
    /// Payment has been authorized (funds reserved).
    Authorized,
    /// Payment has been captured (funds transferred).
    Captured,
    /// Payment has partially been captured.
    PartiallyCaptured,
    /// Payment has failed.
    Failed,
    /// Payment has been cancelled/voided.
    Cancelled,
    /// Payment has been refunded.
    Refunded,
    /// Payment has been partially refunded.
    PartiallyRefunded,
    /// A chargeback/dispute has been opened.
    Disputed,
    /// Payment has expired (e.g., authorization timeout).
    Expired,
}

impl PaymentState {
    /// Returns true if this is a terminal state.
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            PaymentState::Captured
                | PaymentState::Failed
                | PaymentState::Cancelled
                | PaymentState::Refunded
                | PaymentState::Expired
        )
    }

    /// Returns the valid next states from the current state.
    pub fn valid_transitions(&self) -> &[PaymentState] {
        match self {
            PaymentState::Created => &[
                PaymentState::Processing,
                PaymentState::Cancelled,
                PaymentState::Expired,
            ],
            PaymentState::Processing => &[
                PaymentState::Authorized,
                PaymentState::Captured,
                PaymentState::Failed,
            ],
            PaymentState::Authorized => &[
                PaymentState::Captured,
                PaymentState::PartiallyCaptured,
                PaymentState::Cancelled,
                PaymentState::Expired,
            ],
            PaymentState::Captured => &[
                PaymentState::Refunded,
                PaymentState::PartiallyRefunded,
                PaymentState::Disputed,
            ],
            PaymentState::PartiallyCaptured => &[
                PaymentState::Captured,
                PaymentState::Refunded,
                PaymentState::PartiallyRefunded,
            ],
            PaymentState::PartiallyRefunded => &[PaymentState::Refunded, PaymentState::Disputed],
            PaymentState::Failed | PaymentState::Cancelled | PaymentState::Expired => &[],
            PaymentState::Refunded => &[],
            PaymentState::Disputed => &[PaymentState::Refunded, PaymentState::Captured],
        }
    }

    /// Check if transitioning to `next` is valid.
    pub fn can_transition_to(&self, next: PaymentState) -> bool {
        self.valid_transitions().contains(&next)
    }
}

/// Payment status includes the state plus metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentStatus {
    pub payment_id: PaymentId,
    pub state: PaymentState,
    pub amount: Money,
    pub provider_id: Option<ProviderId>,
    pub provider_reference: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

/// Payment method types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    Card,
    BankTransfer,
    DirectDebit,
    Wallet,
    Crypto,
    BuyNowPayLater,
    Voucher,
    Other(String),
}
