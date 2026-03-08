//! # lk-types
//!
//! Canonical domain types for payment infrastructure.
//!
//! This crate provides the foundational types used across all ledgerkit crates:
//! payment states, event types, money/currency types, provider error categories,
//! and webhook verification results.

pub mod currency;
pub mod error;
pub mod event;
pub mod money;
pub mod payment;
pub mod provider;
pub mod webhook;

pub use currency::Currency;
pub use error::{ErrorCategory, LkError};
pub use event::{CanonicalEvent, EventKind};
pub use money::Money;
pub use payment::{PaymentId, PaymentMethod, PaymentState, PaymentStatus};
pub use provider::{ProviderId, ProviderCapability};
pub use webhook::{RawWebhook, VerificationResult, WebhookId};
