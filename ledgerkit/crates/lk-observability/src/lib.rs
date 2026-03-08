//! # lk-observability
//!
//! Observability primitives for payment infrastructure.
//! Provides tracing conventions, correlation IDs, and redaction-safe logging.

pub mod correlation;
pub mod redact;
pub mod setup;

pub use correlation::CorrelationId;
pub use redact::RedactedValue;
pub use setup::init_tracing;
