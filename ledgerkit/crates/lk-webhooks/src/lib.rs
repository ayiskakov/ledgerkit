//! # lk-webhooks
//!
//! Webhook ingestion and verification pipeline for payment providers.
//!
//! Provides HMAC signature verification, timestamp tolerance checks,
//! replay detection hooks, and canonical event output.

pub mod hmac_verifier;
pub mod processor;
pub mod timestamp;

pub use hmac_verifier::HmacVerifier;
pub use processor::WebhookProcessor;
pub use timestamp::TimestampValidator;
