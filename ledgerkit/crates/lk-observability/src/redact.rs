use serde::Serialize;

/// A wrapper that redacts sensitive values in logs and debug output.
///
/// The actual value is preserved for runtime use but hidden in Display/Debug.
#[derive(Clone, Serialize)]
pub struct RedactedValue<T> {
    #[serde(skip)]
    inner: T,
    #[serde(rename = "redacted")]
    _marker: bool,
}

impl<T> RedactedValue<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: value,
            _marker: true,
        }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T> std::fmt::Debug for RedactedValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[REDACTED]")
    }
}

impl<T> std::fmt::Display for RedactedValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[REDACTED]")
    }
}

/// Redact a card number, showing only last 4 digits.
pub fn redact_card(number: &str) -> String {
    if number.len() < 4 {
        return "****".to_string();
    }
    let visible = &number[number.len() - 4..];
    format!("****{}", visible)
}

/// Redact an email address, showing only domain.
pub fn redact_email(email: &str) -> String {
    match email.split_once('@') {
        Some((_, domain)) => format!("***@{}", domain),
        None => "***".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redact_card() {
        assert_eq!(redact_card("4111111111111111"), "****1111");
        assert_eq!(redact_card("12"), "****");
    }

    #[test]
    fn test_redact_email() {
        assert_eq!(redact_email("user@example.com"), "***@example.com");
    }

    #[test]
    fn test_redacted_value_display() {
        let secret = RedactedValue::new("my_api_key");
        assert_eq!(format!("{}", secret), "[REDACTED]");
        assert_eq!(format!("{:?}", secret), "[REDACTED]");
        assert_eq!(*secret.as_ref(), "my_api_key");
    }
}
