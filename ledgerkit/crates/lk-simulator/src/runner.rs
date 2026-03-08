use crate::fixture::Fixture;
use lk_types::event::CanonicalEvent;
use tracing::info;

/// Runs fixture scenarios and collects resulting events.
pub struct SimulatorRunner {
    fixtures: Vec<Fixture>,
}

impl SimulatorRunner {
    pub fn new() -> Self {
        Self {
            fixtures: Vec::new(),
        }
    }

    pub fn add_fixture(&mut self, fixture: Fixture) {
        self.fixtures.push(fixture);
    }

    /// Run all loaded fixtures and return the generated events.
    pub async fn run_all(&self) -> Vec<CanonicalEvent> {
        let mut all_events = Vec::new();

        for fixture in &self.fixtures {
            info!(fixture = %fixture.name, "running fixture");
            let events = fixture.to_canonical_events();

            for (i, event) in events.iter().enumerate() {
                if let Some(delay) = fixture.events.get(i).and_then(|fe| fe.delay_ms) {
                    tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
                }
                info!(
                    event_id = %event.event_id,
                    kind = %event.kind,
                    "generated event"
                );
            }

            all_events.extend(events);
        }

        all_events
    }

    /// List available built-in fixtures.
    pub fn builtin_fixtures() -> Vec<Fixture> {
        vec![
            Fixture::successful_payment(5000, lk_types::currency::Currency::USD),
            Fixture::failed_payment(10000, lk_types::currency::Currency::USD),
            Fixture::refunded_payment(7500, lk_types::currency::Currency::EUR),
        ]
    }
}

impl Default for SimulatorRunner {
    fn default() -> Self {
        Self::new()
    }
}
