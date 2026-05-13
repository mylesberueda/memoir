//! Crypto lifecycle hook.

use super::{Hooks, HooksError};

use common_rs::crypto::LocalCrypto;

pub(crate) struct Crypto {}

impl Crypto {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) async fn init() -> Result<(), HooksError> {
        Self::new().on_startup().await
    }
}

impl Hooks<()> for Crypto {
    async fn on_startup(&self) -> Result<(), HooksError> {
        let passphrase = std::env::var("CRYPTO_PASSPHRASE")
            .map_err(|_| HooksError::Config("CRYPTO_PASSPHRASE must be set".to_string()))?;

        let salt =
            std::env::var("CRYPTO_SALT").map_err(|_| HooksError::Config("CRYPTO_SALT must be set".to_string()))?;

        let crypto = LocalCrypto::from_passphrase(&passphrase, &salt)
            .map_err(|e| HooksError::Config(format!("Failed to initialize crypto: {e}")))?;

        crate::api::crypto::init(crypto);
        tracing::info!("Crypto initialized!");

        Ok(())
    }
}
