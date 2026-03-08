use serde::{Deserialize, Serialize};

/// Unique identifier for a payment provider/PSP.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProviderId(pub String);

impl ProviderId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl std::fmt::Display for ProviderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Capability flags for a payment provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderCapability {
    /// Supports authorization + capture as separate steps
    AuthCapture,
    /// Supports single-step payment (direct capture)
    DirectCapture,
    /// Supports partial captures
    PartialCapture,
    /// Supports refunds
    Refund,
    /// Supports partial refunds
    PartialRefund,
    /// Supports void/cancel of authorized payments
    Void,
    /// Supports webhook notifications
    Webhooks,
    /// Supports 3DS authentication
    ThreeDSecure,
    /// Supports recurring/subscription payments
    Recurring,
    /// Supports payouts
    Payouts,
    /// Supports tokenization
    Tokenization,
    /// Supports multi-currency
    MultiCurrency,
}
