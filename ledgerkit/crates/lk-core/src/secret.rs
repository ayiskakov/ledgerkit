use async_trait::async_trait;

/// Trait for accessing secrets (API keys, webhook signing keys, etc.).
///
/// Abstracts over different secret storage backends (env vars, vault, etc.)
#[async_trait]
pub trait SecretProvider: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Retrieve a secret by name.
    async fn get_secret(&self, name: &str) -> Result<String, Self::Error>;

    /// Check if a secret exists.
    async fn has_secret(&self, name: &str) -> Result<bool, Self::Error>;
}

/// Simple environment variable based secret provider.
#[derive(Debug, Clone)]
pub struct EnvSecretProvider {
    prefix: Option<String>,
}

impl EnvSecretProvider {
    pub fn new() -> Self {
        Self { prefix: None }
    }

    pub fn with_prefix(prefix: &str) -> Self {
        Self {
            prefix: Some(prefix.to_string()),
        }
    }

    fn key_name(&self, name: &str) -> String {
        match &self.prefix {
            Some(prefix) => format!("{}_{}", prefix, name.to_uppercase()),
            None => name.to_uppercase(),
        }
    }
}

impl Default for EnvSecretProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SecretError {
    #[error("secret not found: {0}")]
    NotFound(String),
}

#[async_trait]
impl SecretProvider for EnvSecretProvider {
    type Error = SecretError;

    async fn get_secret(&self, name: &str) -> Result<String, Self::Error> {
        let key = self.key_name(name);
        std::env::var(&key).map_err(|_| SecretError::NotFound(key))
    }

    async fn has_secret(&self, name: &str) -> Result<bool, Self::Error> {
        let key = self.key_name(name);
        Ok(std::env::var(&key).is_ok())
    }
}
