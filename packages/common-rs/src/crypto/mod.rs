pub mod hashing;
mod local;

pub use local::LocalCrypto;

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("encryption failed")]
    EncryptionFailed,
    #[error("decryption failed (invalid key or tampered data)")]
    DecryptionFailed,
    #[error("missing key: {0}")]
    MissingKey(String),
    #[error("key derivation failed")]
    KeyDerivationFailed,
    #[error("invalid ciphertext (too short)")]
    InvalidCipherText,
    #[error("invalid key")]
    InvalidKey,
    #[error("password hashing failed")]
    HashFailed,
    #[error("password verification failed (malformed hash)")]
    VerifyFailed,
    #[error("invalid API key format")]
    InvalidKeyFormat,
    #[error("RNG failed to produce random bytes")]
    RngFailed,
}

/// Trait for types that can be displayed in a truncated/redacted form.
/// Useful for showing secrets in UI where users need to identify which key
/// is configured without exposing the full value.
pub trait Redactable {
    /// Returns a truncated view safe for display (e.g., `***xyz12345`)
    fn redacted(&self) -> String;
}

pub struct Secret(String);

impl Secret {
    /// Use the secret value
    pub fn expose(&self) -> &str {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Redactable for Secret {
    /// Returns a fixed 12-character truncated view for UI display.
    /// Shows `***` prefix followed by the last 25% of the secret (up to 9 chars).
    ///
    /// Returns `"***"` if the secret is 4 characters or shorter.
    fn redacted(&self) -> String {
        let len = self.0.len();
        if len <= 4 {
            "***".to_string()
        } else {
            let visible = (len / 4).min(9); // Cap at 9 to keep total at 12
            format!("***{}", &self.0[len - visible..])
        }
    }
}

impl std::fmt::Display for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[redacted]")
    }
}

impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Secret").field(&"[redacted]").finish()
    }
}

pub trait SecretCrypto: Send + Sync + 'static {
    /// Encrypt plaintext bytes
    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError>;

    /// Decrypt ciphertext bytes
    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError>;

    /// Decrypt to Secret
    fn decrypt_to_secret(&self, ciphertext: &[u8]) -> Result<Secret, CryptoError> {
        let bytes = self.decrypt(ciphertext)?;
        let secret = Secret(String::from_utf8(bytes).map_err(|_| CryptoError::DecryptionFailed)?);

        Ok(secret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_crypto() -> LocalCrypto {
        LocalCrypto::from_passphrase("test-passphrase", "test-salt").unwrap()
    }

    #[test]
    fn should_redact_secret_in_debug_output() {
        let secret = Secret("super-secret-api-key".to_string());

        let debug_output = format!("{:?}", secret);

        assert!(debug_output.contains("[redacted]"));
        assert!(!debug_output.contains("super-secret-api-key"));
    }

    #[test]
    fn should_redact_secret_in_display_output() {
        let secret = Secret("super-secret-api-key".to_string());

        let display_output = format!("{}", secret);

        assert_eq!(display_output, "[redacted]");
    }

    #[test]
    fn should_expose_secret_value_explicitly() {
        let secret = Secret("my-api-key".to_string());

        assert_eq!(secret.expose(), "my-api-key");
    }

    #[test]
    fn should_redact_capped_at_12_total() {
        // 40 char key -> 25% = 10, but capped at 9 for 12-char total
        let secret = Secret("sk-1234567890abcdefghijklmnopqrstuvwxyz12".to_string());

        assert_eq!(secret.redacted(), "***tuvwxyz12");
        assert_eq!(secret.redacted().len(), 12);
    }

    #[test]
    fn should_redact_short_key() {
        // 16 char key -> shows last 4
        let secret = Secret("sk-test-key-1234".to_string());

        assert_eq!(secret.redacted(), "***1234");
    }

    #[test]
    fn should_redact_very_short_secrets() {
        let secret = Secret("abc".to_string());

        assert_eq!(secret.redacted(), "***");
    }

    #[test]
    fn should_redact_boundary_length() {
        // Exactly 4 chars should be fully redacted
        let secret = Secret("abcd".to_string());

        assert_eq!(secret.redacted(), "***");
    }

    #[test]
    fn should_redact_5_char_secret() {
        // 5 chars -> 5/4 = 1 visible
        let secret = Secret("abcde".to_string());

        assert_eq!(secret.redacted(), "***e");
    }

    #[test]
    fn should_decrypt_to_secret_and_expose_value() {
        let crypto = test_crypto();
        let original = "api-key-12345";

        let ciphertext = crypto.encrypt(original.as_bytes()).unwrap();
        let secret = crypto.decrypt_to_secret(&ciphertext).unwrap();

        assert_eq!(secret.expose(), original);
    }

    #[test]
    fn should_fail_decrypt_to_secret_with_invalid_utf8() {
        let crypto = test_crypto();
        // Invalid UTF-8 sequence
        let invalid_utf8 = vec![0xFF, 0xFE, 0x00, 0x01];

        let ciphertext = crypto.encrypt(&invalid_utf8).unwrap();
        let result = crypto.decrypt_to_secret(&ciphertext);

        assert!(matches!(result, Err(CryptoError::DecryptionFailed)));
    }
}
