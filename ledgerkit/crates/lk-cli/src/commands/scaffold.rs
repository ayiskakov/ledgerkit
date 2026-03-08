use anyhow::Result;

pub fn handle(name: &str) -> Result<()> {
    println!("Scaffolding new connector: {}", name);
    println!();
    println!("Create the following structure:");
    println!("  crates/lk-connectors/src/{}.rs", name);
    println!();
    println!("Implement these traits:");
    println!("  - PaymentConnector (authorize, capture, refund, parse_webhook)");
    println!("  - WebhookVerifier (verify)");
    println!();
    println!("See crates/lk-connectors/src/mock.rs for a reference implementation.");
    Ok(())
}
