//! # lk-connectors
//!
//! Payment provider connector implementations.
//!
//! This crate provides concrete implementations of the `PaymentConnector` trait
//! for various payment providers. Start with the `MockConnector` for testing.

pub mod mock;

pub use mock::MockConnector;
