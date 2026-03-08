//! # lk-store
//!
//! Storage adapters for ledgerkit.
//! Provides in-memory implementations for development and testing.

pub mod memory;

pub use memory::InMemoryIdempotencyStore;
pub use memory::InMemoryEventStore;
