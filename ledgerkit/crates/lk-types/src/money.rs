use crate::currency::Currency;
use serde::{Deserialize, Serialize};

/// Represents a monetary amount in minor units (e.g., cents for USD).
///
/// All amounts are stored as integers in the smallest currency unit
/// to avoid floating-point precision issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    /// Amount in minor units (e.g., 1000 = $10.00 for USD)
    pub amount: i64,
    /// Currency of the amount
    pub currency: Currency,
}

impl Money {
    /// Create a new Money value from minor units.
    pub fn new(amount: i64, currency: Currency) -> Self {
        Self { amount, currency }
    }

    /// Create a Money value from a major unit amount (e.g., dollars).
    pub fn from_major(amount: f64, currency: Currency) -> Self {
        let minor_units = currency.minor_units();
        let factor = 10_i64.pow(minor_units as u32);
        Self {
            amount: (amount * factor as f64).round() as i64,
            currency,
        }
    }

    /// Returns the amount as a major unit float (e.g., dollars).
    pub fn to_major(&self) -> f64 {
        let factor = 10_f64.powi(self.currency.minor_units() as i32);
        self.amount as f64 / factor
    }

    /// Returns true if the amount is zero.
    pub fn is_zero(&self) -> bool {
        self.amount == 0
    }

    /// Returns true if the amount is positive.
    pub fn is_positive(&self) -> bool {
        self.amount > 0
    }

    /// Returns true if the amount is negative.
    pub fn is_negative(&self) -> bool {
        self.amount < 0
    }

    /// Returns the absolute value.
    pub fn abs(&self) -> Self {
        Self {
            amount: self.amount.abs(),
            currency: self.currency,
        }
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let minor_units = self.currency.minor_units();
        if minor_units == 0 {
            write!(f, "{} {}", self.amount, self.currency)
        } else {
            let factor = 10_i64.pow(minor_units as u32);
            let major = self.amount / factor;
            let minor = (self.amount % factor).abs();
            write!(
                f,
                "{}.{:0>width$} {}",
                major,
                minor,
                self.currency,
                width = minor_units as usize
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_display() {
        let m = Money::new(1050, Currency::USD);
        assert_eq!(m.to_string(), "10.50 USD");
    }

    #[test]
    fn test_money_from_major() {
        let m = Money::from_major(10.50, Currency::USD);
        assert_eq!(m.amount, 1050);
    }

    #[test]
    fn test_money_jpy_no_decimals() {
        let m = Money::new(1000, Currency::JPY);
        assert_eq!(m.to_string(), "1000 JPY");
    }
}
