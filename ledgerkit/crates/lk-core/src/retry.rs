use std::time::Duration;

/// Decision from a retry policy evaluation.
#[derive(Debug, Clone)]
pub enum RetryDecision {
    /// Retry the operation after the specified delay.
    Retry { delay: Duration, attempt: u32 },
    /// Do not retry; all attempts exhausted or error is not retryable.
    GiveUp { attempts_made: u32, reason: String },
}

/// Configuration for retry behavior.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts.
    pub max_retries: u32,
    /// Initial delay before the first retry.
    pub initial_delay: Duration,
    /// Maximum delay between retries.
    pub max_delay: Duration,
    /// Multiplier applied to the delay after each retry.
    pub backoff_factor: f64,
    /// Whether to add jitter to the delay.
    pub jitter: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(30),
            backoff_factor: 2.0,
            jitter: true,
        }
    }
}

impl RetryPolicy {
    /// Create a new retry policy with the given max retries.
    pub fn new(max_retries: u32) -> Self {
        Self {
            max_retries,
            ..Default::default()
        }
    }

    /// No retries at all.
    pub fn none() -> Self {
        Self {
            max_retries: 0,
            ..Default::default()
        }
    }

    /// Evaluate whether to retry based on the current attempt number.
    pub fn evaluate(&self, attempt: u32) -> RetryDecision {
        if attempt >= self.max_retries {
            return RetryDecision::GiveUp {
                attempts_made: attempt,
                reason: format!("max retries ({}) exhausted", self.max_retries),
            };
        }

        let mut delay_ms = self.initial_delay.as_millis() as f64
            * self.backoff_factor.powi(attempt as i32);

        if delay_ms > self.max_delay.as_millis() as f64 {
            delay_ms = self.max_delay.as_millis() as f64;
        }

        if self.jitter {
            // Simple jitter: randomize between 50% and 100% of computed delay.
            // Using a deterministic approach for now (no rand dependency).
            let jitter_factor = 0.5 + (((attempt as f64 * 7.0) % 10.0) / 20.0);
            delay_ms *= jitter_factor;
        }

        RetryDecision::Retry {
            delay: Duration::from_millis(delay_ms as u64),
            attempt: attempt + 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_policy() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.max_retries, 3);

        match policy.evaluate(0) {
            RetryDecision::Retry { attempt, .. } => assert_eq!(attempt, 1),
            _ => panic!("Expected retry"),
        }
    }

    #[test]
    fn test_give_up_after_max() {
        let policy = RetryPolicy::new(2);
        match policy.evaluate(2) {
            RetryDecision::GiveUp { attempts_made, .. } => assert_eq!(attempts_made, 2),
            _ => panic!("Expected give up"),
        }
    }

    #[test]
    fn test_no_retries() {
        let policy = RetryPolicy::none();
        match policy.evaluate(0) {
            RetryDecision::GiveUp { .. } => {}
            _ => panic!("Expected give up"),
        }
    }
}
