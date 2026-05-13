use common_rs::crypto::SecretCrypto;
use std::sync::OnceLock;

static CRYPTO: OnceLock<Box<dyn SecretCrypto>> = OnceLock::new();

pub fn init(crypto: impl SecretCrypto + 'static) {
    if CRYPTO.set(Box::new(crypto)).is_err() {
        tracing::error!("Crypto already initialized. Are you calling crypto::init twice?")
    }
}

pub(crate) fn get_crypto() -> &'static dyn SecretCrypto {
    CRYPTO.get().expect("Crypto not initialized").as_ref()
}
