use anyhow::Result;
use lk_webhooks::hmac_verifier::HmacVerifier;
use lk_core::webhook::WebhookVerifier;
use lk_types::webhook::RawWebhook;
use std::collections::HashMap;

pub fn handle(payload_path: &str, secret: &str, signature: &str) -> Result<()> {
    let body = std::fs::read_to_string(payload_path)?;
    let verifier = HmacVerifier::hex(secret.as_bytes(), "x-signature");

    let mut headers = HashMap::new();
    headers.insert("x-signature".to_string(), signature.to_string());

    let webhook = RawWebhook::new(headers, body);
    let result = verifier.verify(&webhook)?;

    if result.is_valid() {
        println!("Signature is VALID");
    } else {
        println!("Signature is INVALID: {:?}", result);
    }

    Ok(())
}
