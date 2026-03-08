//! # lk-core
//!
//! Core traits and abstractions for payment infrastructure.
//!
//! This crate provides the trait definitions that all ledgerkit components implement:
//! payment connectors, webhook verifiers, idempotency stores, retry policies,
//! and other foundational abstractions.

pub mod clock;
pub mod connector;
pub mod idempotency;
pub mod retry;
pub mod secret;
pub mod webhook;

pub use clock::Clock;
pub use connector::PaymentConnector;
pub use idempotency::IdempotencyStore;
pub use retry::{RetryPolicy, RetryDecision};
pub use secret::SecretProvider;
pub use webhook::WebhookVerifier;
