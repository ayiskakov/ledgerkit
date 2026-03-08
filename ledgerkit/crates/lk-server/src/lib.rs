//! # lk-server
//!
//! Optional HTTP server wrapper for ledgerkit.
//! Provides webhook endpoints, admin/dev endpoints, and health checks.

pub mod health;
pub mod routes;

pub use routes::create_router;
