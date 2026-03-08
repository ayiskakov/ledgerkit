use chrono::{DateTime, Utc};

/// Abstraction over time for testability.
///
/// In production, use `SystemClock`. In tests, use `MockClock`
/// to control time deterministically.
pub trait Clock: Send + Sync {
    /// Returns the current UTC timestamp.
    fn now(&self) -> DateTime<Utc>;
}

/// Real system clock.
#[derive(Debug, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

/// Mock clock for testing with a fixed or controllable time.
#[derive(Debug, Clone)]
pub struct MockClock {
    now: std::sync::Arc<std::sync::Mutex<DateTime<Utc>>>,
}

impl MockClock {
    pub fn new(time: DateTime<Utc>) -> Self {
        Self {
            now: std::sync::Arc::new(std::sync::Mutex::new(time)),
        }
    }

    pub fn advance(&self, duration: chrono::Duration) {
        let mut now = self.now.lock().unwrap();
        *now = *now + duration;
    }

    pub fn set(&self, time: DateTime<Utc>) {
        let mut now = self.now.lock().unwrap();
        *now = time;
    }
}

impl Clock for MockClock {
    fn now(&self) -> DateTime<Utc> {
        *self.now.lock().unwrap()
    }
}
