use serde::{Deserialize, Serialize};

/// ISO 4217 currency codes commonly used in payment processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Currency {
    USD,
    EUR,
    GBP,
    JPY,
    CHF,
    CAD,
    AUD,
    CNY,
    INR,
    BRL,
    MXN,
    SEK,
    NOK,
    DKK,
    PLN,
    CZK,
    HUF,
    SGD,
    HKD,
    NZD,
    /// Bitcoin (non-ISO but widely used)
    BTC,
    /// Ethereum (non-ISO but widely used)
    ETH,
    /// USDT stablecoin
    USDT,
    /// USDC stablecoin
    USDC,
    /// Other currency represented by its 3-letter code
    #[serde(untagged)]
    Other(CurrencyCode),
}

/// A 3-letter currency code for currencies not in the enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CurrencyCode([u8; 3]);

impl CurrencyCode {
    pub fn new(code: &str) -> Option<Self> {
        if code.len() != 3 || !code.chars().all(|c| c.is_ascii_uppercase()) {
            return None;
        }
        let mut bytes = [0u8; 3];
        bytes.copy_from_slice(code.as_bytes());
        Some(Self(bytes))
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).unwrap()
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Currency::USD => write!(f, "USD"),
            Currency::EUR => write!(f, "EUR"),
            Currency::GBP => write!(f, "GBP"),
            Currency::JPY => write!(f, "JPY"),
            Currency::CHF => write!(f, "CHF"),
            Currency::CAD => write!(f, "CAD"),
            Currency::AUD => write!(f, "AUD"),
            Currency::CNY => write!(f, "CNY"),
            Currency::INR => write!(f, "INR"),
            Currency::BRL => write!(f, "BRL"),
            Currency::MXN => write!(f, "MXN"),
            Currency::SEK => write!(f, "SEK"),
            Currency::NOK => write!(f, "NOK"),
            Currency::DKK => write!(f, "DKK"),
            Currency::PLN => write!(f, "PLN"),
            Currency::CZK => write!(f, "CZK"),
            Currency::HUF => write!(f, "HUF"),
            Currency::SGD => write!(f, "SGD"),
            Currency::HKD => write!(f, "HKD"),
            Currency::NZD => write!(f, "NZD"),
            Currency::BTC => write!(f, "BTC"),
            Currency::ETH => write!(f, "ETH"),
            Currency::USDT => write!(f, "USDT"),
            Currency::USDC => write!(f, "USDC"),
            Currency::Other(code) => write!(f, "{}", code.as_str()),
        }
    }
}

impl Currency {
    /// Returns the number of minor unit digits (e.g., 2 for USD cents, 0 for JPY).
    pub fn minor_units(&self) -> u8 {
        match self {
            Currency::JPY | Currency::HUF => 0,
            Currency::BTC => 8,
            Currency::ETH => 18,
            _ => 2,
        }
    }
}
