use anyhow::Result;
use lk_simulator::runner::SimulatorRunner;

pub async fn handle(fixture: Option<String>, all: bool) -> Result<()> {
    let mut runner = SimulatorRunner::new();

    if all {
        for f in SimulatorRunner::builtin_fixtures() {
            runner.add_fixture(f);
        }
    } else if let Some(name) = fixture {
        let fixtures = SimulatorRunner::builtin_fixtures();
        let f = fixtures
            .into_iter()
            .find(|f| f.name == name)
            .ok_or_else(|| anyhow::anyhow!("fixture not found: {}", name))?;
        runner.add_fixture(f);
    } else {
        runner.add_fixture(lk_simulator::fixture::Fixture::successful_payment(
            5000,
            lk_types::currency::Currency::USD,
        ));
    }

    let events = runner.run_all().await;
    println!("\nSimulation complete. Generated {} events:", events.len());
    for event in &events {
        println!(
            "  [{}] {} - {:?}",
            event.kind, event.event_id, event.amount
        );
    }

    Ok(())
}
