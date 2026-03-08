use anyhow::Result;
use lk_simulator::fixture::Fixture;
use lk_types::currency::Currency;

pub async fn handle(action: crate::FixtureAction) -> Result<()> {
    match action {
        crate::FixtureAction::List => {
            println!("Available built-in fixtures:");
            println!("  successful  - Standard authorize + capture flow");
            println!("  failed      - Payment that fails during authorization");
            println!("  refunded    - Captured payment that is fully refunded");
        }
        crate::FixtureAction::Generate {
            kind,
            amount,
            currency: _,
            output,
        } => {
            let fixture = match kind.as_str() {
                "successful" => Fixture::successful_payment(amount, Currency::USD),
                "failed" => Fixture::failed_payment(amount, Currency::USD),
                "refunded" => Fixture::refunded_payment(amount, Currency::USD),
                other => anyhow::bail!("unknown fixture kind: {}", other),
            };

            let json = serde_json::to_string_pretty(&fixture)?;
            match output {
                Some(path) => {
                    std::fs::write(&path, &json)?;
                    println!("Fixture written to {}", path);
                }
                None => println!("{}", json),
            }
        }
        crate::FixtureAction::Replay { path } => {
            let content = std::fs::read_to_string(&path)?;
            let fixture: Fixture = serde_json::from_str(&content)?;
            let events = fixture.to_canonical_events();
            for event in &events {
                println!(
                    "  {} | {} | {:?}",
                    event.event_id, event.kind, event.amount
                );
            }
            println!("\nReplayed {} events", events.len());
        }
    }
    Ok(())
}
