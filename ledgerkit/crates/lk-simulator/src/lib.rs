//! # lk-simulator
//!
//! Local payment event simulation engine.
//! Generate test fixtures, replay webhooks, and run deterministic scenarios.

pub mod fixture;
pub mod runner;

pub use fixture::{Fixture, FixtureEvent};
pub use runner::SimulatorRunner;
