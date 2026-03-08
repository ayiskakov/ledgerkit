use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "lk", version, about = "LedgerKit CLI - Payment infrastructure developer tools")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate and manage test fixtures
    Fixture {
        #[command(subcommand)]
        action: FixtureAction,
    },
    /// Run the local payment simulator
    Simulate {
        /// Fixture name to simulate
        #[arg(short, long)]
        fixture: Option<String>,

        /// Run all built-in fixtures
        #[arg(long)]
        all: bool,
    },
    /// Verify a webhook payload signature
    Verify {
        /// Path to the payload file
        #[arg(short, long)]
        payload: String,

        /// Signing secret
        #[arg(short, long)]
        secret: String,

        /// Signature to verify
        #[arg(long)]
        signature: String,
    },
    /// Scaffold a new connector
    Scaffold {
        /// Name of the connector
        name: String,
    },
}

#[derive(Subcommand)]
pub enum FixtureAction {
    /// List available fixtures
    List,
    /// Generate a fixture file
    Generate {
        /// Fixture type: successful, failed, refunded
        #[arg(short, long, default_value = "successful")]
        kind: String,

        /// Amount in minor units
        #[arg(short, long, default_value = "5000")]
        amount: i64,

        /// Currency code
        #[arg(short, long, default_value = "USD")]
        currency: String,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Replay a fixture file
    Replay {
        /// Path to the fixture JSON file
        path: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    lk_observability::init_tracing();
    let cli = Cli::parse();

    match cli.command {
        Commands::Fixture { action } => commands::fixture::handle(action).await?,
        Commands::Simulate { fixture, all } => commands::simulate::handle(fixture, all).await?,
        Commands::Verify {
            payload,
            secret,
            signature,
        } => commands::verify::handle(&payload, &secret, &signature)?,
        Commands::Scaffold { name } => commands::scaffold::handle(&name)?,
    }

    Ok(())
}
