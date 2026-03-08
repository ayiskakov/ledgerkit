# LedgerKit

An open-core Rust toolkit for building payment connectors, webhook processors, and transaction event pipelines with strong typing, replay safety, and production-grade observability.

## Why LedgerKit?

Teams building payment infrastructure repeat the same painful work: connector boilerplate for dozens of PSPs, inconsistent webhook verification, idempotency handling, retry logic, and event normalization across gateways. Rust has general-purpose crates, but no opinionated toolkit for payment infrastructure.

LedgerKit fills that gap.

## Features

- **Connector SDK** — Standard traits and helpers for integrating PSPs/acquirers with request/response normalization, auth abstraction, retry/backoff, and error mapping
- **Webhook Engine** — Signature verification, timestamp tolerance, replay protection, idempotency, and event normalization
- **Payment Event Model** — Canonical event schema for the full payment lifecycle (`payment.created`, `payment.authorized`, `payment.captured`, `payment.refunded`, `chargeback.opened`, etc.)
- **Local Simulator & CLI** — Simulate payment events locally, replay webhooks, validate signatures, and inspect payload mappings
- **Observability Primitives** — Correlation IDs, transaction timeline spans, provider latency metrics, and redaction-safe structured logging

## Architecture

LedgerKit is organized as a Rust workspace with focused crates:

```
ledgerkit/
  crates/
    lk-types/          # Canonical enums, domain models, money/currency types
    lk-core/           # Connector trait, webhook verifier trait, retry policy
    lk-connectors/     # Provider implementations and adapter helpers
    lk-webhooks/       # Webhook ingestion and verification pipeline
    lk-store/          # Storage adapters (in-memory, Postgres, Redis)
    lk-simulator/      # Local event generation and fixture runner
    lk-cli/            # Developer CLI
    lk-observability/  # Tracing, logging, and metrics wrappers
    lk-server/         # Optional HTTP service wrapper (axum)
    lk-examples/       # Example integrations
```

## Quick Start

### Prerequisites

- Rust 1.75+

### Build

```bash
cd ledgerkit
cargo build
```

### Run Tests

```bash
cargo test
```

### CLI

```bash
cargo run -p lk-cli -- --help
```

## Usage

### Implementing a Payment Connector

```rust
use async_trait::async_trait;

#[async_trait]
pub trait PaymentConnector {
    type Config;
    type Error;

    async fn authorize(&self, req: AuthorizeRequest) -> Result<AuthorizeResponse, Self::Error>;
    async fn capture(&self, req: CaptureRequest) -> Result<CaptureResponse, Self::Error>;
    async fn refund(&self, req: RefundRequest) -> Result<RefundResponse, Self::Error>;
    async fn parse_webhook(&self, input: RawWebhook) -> Result<CanonicalEvent, Self::Error>;
}
```

### Verifying Webhooks

```rust
pub trait WebhookVerifier {
    type Error;

    fn verify(&self, headers: &HeaderMap, body: &[u8]) -> Result<VerificationResult, Self::Error>;
}
```

## Tech Stack

- **Runtime:** Tokio
- **HTTP:** Axum, Reqwest, Tower
- **Serialization:** Serde
- **Observability:** tracing, OpenTelemetry
- **CLI:** Clap
- **Crypto:** HMAC-SHA256 for webhook signatures
- **Testing:** wiremock

## License

Licensed under Apache License 2.0. See [LICENSE](LICENSE) for details.
