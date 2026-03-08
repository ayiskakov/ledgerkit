use chrono::{DateTime, Utc};
use lk_types::webhook::VerificationResult;

/// Validates that webhook timestamps are within an acceptable window.
#[derive(Debug, Clone)]
pub struct TimestampValidator {
    /// Maximum age of a webhook in seconds.
    pub tolerance_secs: u64,
}

impl TimestampValidator {
    pub fn new(tolerance_secs: u64) -> Self {
        Self { tolerance_secs }
    }

    /// Default tolerance of 5 minutes.
    pub fn default_tolerance() -> Self {
        Self::new(300)
    }

    /// Validate a timestamp against the current time.
    pub fn validate(&self, timestamp: &DateTime<Utc>) -> VerificationResult {
        let now = Utc::now();
        let age = now.signed_duration_since(*timestamp);

        if age.num_seconds().unsigned_abs() > self.tolerance_secs {
            VerificationResult::TimestampExpired {
                received: timestamp.to_rfc3339(),
                tolerance_secs: self.tolerance_secs,
            }
        } else {
            VerificationResult::Valid
        }
    }

    /// Parse a unix timestamp and validate it.
    pub fn validate_unix(&self, unix_ts: i64) -> VerificationResult {
        match DateTime::from_timestamp(unix_ts, 0) {
            Some(ts) => self.validate(&ts),
            None => VerificationResult::Invalid {
                reason: format!("invalid unix timestamp: {}", unix_ts),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_timestamp() {
        let validator = TimestampValidator::default_tolerance();
        let now = Utc::now();
        assert!(validator.validate(&now).is_valid());
    }

    #[test]
    fn test_expired_timestamp() {
        let validator = TimestampValidator::new(60);
        let old = Utc::now() - chrono::Duration::seconds(120);
        let result = validator.validate(&old);
        assert!(matches!(result, VerificationResult::TimestampExpired { .. }));
    }
}
