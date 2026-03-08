use lk_types::webhook::{RawWebhook, VerificationResult};

/// Trait for verifying webhook signatures from payment providers.
///
/// Each provider has its own signing mechanism (HMAC-SHA256, RSA, etc.).
/// Implementations of this trait handle the provider-specific verification logic.
pub trait WebhookVerifier: Send + Sync {
    /// The error type for verification failures.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Verify the authenticity of an incoming webhook.
    ///
    /// Checks the signature, timestamp tolerance, and any provider-specific
    /// validation rules.
    fn verify(&self, webhook: &RawWebhook) -> Result<VerificationResult, Self::Error>;
}
