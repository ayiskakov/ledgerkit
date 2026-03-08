use hmac::{Hmac, Mac};
use lk_core::webhook::WebhookVerifier;
use lk_types::webhook::{RawWebhook, VerificationResult};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Configuration for HMAC-SHA256 webhook verification.
#[derive(Debug, Clone)]
pub struct HmacVerifierConfig {
    /// The signing secret.
    pub secret: Vec<u8>,
    /// Header name containing the signature.
    pub signature_header: String,
    /// Optional header name containing the timestamp.
    pub timestamp_header: Option<String>,
    /// Signature encoding (hex or base64).
    pub encoding: SignatureEncoding,
    /// Optional prefix in the signature header value (e.g., "sha256=" for GitHub).
    pub signature_prefix: Option<String>,
}

/// How the signature is encoded in the header.
#[derive(Debug, Clone, Copy)]
pub enum SignatureEncoding {
    Hex,
    Base64,
}

/// HMAC-SHA256 webhook signature verifier.
///
/// This is the most common webhook signing mechanism used by payment providers
/// like Stripe, Adyen, and others.
pub struct HmacVerifier {
    config: HmacVerifierConfig,
}

impl HmacVerifier {
    pub fn new(config: HmacVerifierConfig) -> Self {
        Self { config }
    }

    /// Create a simple HMAC verifier with hex-encoded signatures.
    pub fn hex(secret: &[u8], signature_header: &str) -> Self {
        Self::new(HmacVerifierConfig {
            secret: secret.to_vec(),
            signature_header: signature_header.to_string(),
            timestamp_header: None,
            encoding: SignatureEncoding::Hex,
            signature_prefix: None,
        })
    }

    /// Compute the HMAC-SHA256 signature of a payload.
    pub fn compute_signature(&self, payload: &[u8]) -> Vec<u8> {
        let mut mac =
            HmacSha256::new_from_slice(&self.config.secret).expect("HMAC key should be valid");
        mac.update(payload);
        mac.finalize().into_bytes().to_vec()
    }

    /// Compute and return the hex-encoded signature.
    pub fn sign_hex(&self, payload: &[u8]) -> String {
        hex::encode(self.compute_signature(payload))
    }

    fn extract_signature<'a>(&self, raw: &'a str) -> &'a str {
        match &self.config.signature_prefix {
            Some(prefix) => raw.strip_prefix(prefix.as_str()).unwrap_or(raw),
            None => raw,
        }
    }

    fn decode_signature(&self, encoded: &str) -> Option<Vec<u8>> {
        match self.config.encoding {
            SignatureEncoding::Hex => hex::decode(encoded).ok(),
            SignatureEncoding::Base64 => {
                use sha2::Digest;
                // Simple base64 decode (avoiding extra dependency for now)
                None // TODO: add base64 dependency for base64 support
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HmacVerifierError {
    #[error("missing signature header: {0}")]
    MissingHeader(String),
    #[error("invalid signature encoding")]
    InvalidEncoding,
}

impl WebhookVerifier for HmacVerifier {
    type Error = HmacVerifierError;

    fn verify(&self, webhook: &RawWebhook) -> Result<VerificationResult, Self::Error> {
        let sig_header = webhook
            .header(&self.config.signature_header)
            .ok_or_else(|| {
                HmacVerifierError::MissingHeader(self.config.signature_header.clone())
            })?;

        let sig_str = self.extract_signature(sig_header);
        let expected_bytes = match self.decode_signature(sig_str) {
            Some(bytes) => bytes,
            None => return Err(HmacVerifierError::InvalidEncoding),
        };

        let mut mac =
            HmacSha256::new_from_slice(&self.config.secret).expect("HMAC key should be valid");
        mac.update(webhook.body.as_bytes());

        match mac.verify_slice(&expected_bytes) {
            Ok(()) => Ok(VerificationResult::Valid),
            Err(_) => Ok(VerificationResult::Invalid {
                reason: "HMAC signature mismatch".to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_hmac_sign_and_verify() {
        let secret = b"test_secret_key";
        let verifier = HmacVerifier::hex(secret, "x-webhook-signature");

        let body = r#"{"event":"payment.captured"}"#;
        let signature = verifier.sign_hex(body.as_bytes());

        let mut headers = HashMap::new();
        headers.insert("x-webhook-signature".to_string(), signature);

        let webhook = RawWebhook::new(headers, body.to_string());
        let result = verifier.verify(&webhook).unwrap();
        assert!(result.is_valid());
    }

    #[test]
    fn test_hmac_invalid_signature() {
        let secret = b"test_secret_key";
        let verifier = HmacVerifier::hex(secret, "x-webhook-signature");

        let mut headers = HashMap::new();
        headers.insert("x-webhook-signature".to_string(), "deadbeef".to_string());

        let webhook = RawWebhook::new(headers, "test body".to_string());
        let result = verifier.verify(&webhook).unwrap();
        assert!(result.is_rejected());
    }

    #[test]
    fn test_hmac_missing_header() {
        let secret = b"test_secret_key";
        let verifier = HmacVerifier::hex(secret, "x-webhook-signature");

        let webhook = RawWebhook::new(HashMap::new(), "test body".to_string());
        let result = verifier.verify(&webhook);
        assert!(result.is_err());
    }
}
