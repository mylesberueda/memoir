use super::*;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::{Aead, Generate}};
use argon2::Argon2;

const NONCE_SIZE: usize = 12;
const KEY_SIZE: usize = 32;

pub struct LocalCrypto {
    cipher: Aes256Gcm,
}

impl LocalCrypto {
    pub fn from_passphrase(passphrase: &str, salt: &str) -> Result<Self, CryptoError> {
        let mut key = [0u8; KEY_SIZE];
        Argon2::default()
            .hash_password_into(passphrase.as_bytes(), salt.as_bytes(), &mut key)
            .map_err(|_| CryptoError::KeyDerivationFailed)?;

        Ok(Self {
            cipher: Aes256Gcm::new_from_slice(&key).map_err(|_| CryptoError::InvalidKey)?,
        })
    }
}

impl SecretCrypto for LocalCrypto {
    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let nonce = aes_gcm::Nonce::generate();
        let ciphertext = self
            .cipher
            .encrypt(&nonce, plaintext)
            .map_err(|_| CryptoError::EncryptionFailed)?;

        let mut result = nonce.to_vec();
        result.extend(ciphertext);
        Ok(result)
    }

    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if data.len() < NONCE_SIZE + 16 {
            return Err(CryptoError::InvalidCipherText);
        }

        let (nonce_bytes, ciphertext) = data.split_at(NONCE_SIZE);
        let nonce = Nonce::from_iter(nonce_bytes.iter().copied());
        self.cipher
            .decrypt(&nonce, ciphertext)
            .map_err(|_| CryptoError::DecryptionFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_crypto() -> LocalCrypto {
        LocalCrypto::from_passphrase("test-passphrase", "test-salt").unwrap()
    }

    #[test]
    fn should_encrypt_and_decrypt_roundtrip() {
        let crypto = test_crypto();
        let plaintext = b"hello world";

        let ciphertext = crypto.encrypt(plaintext).unwrap();
        let decrypted = crypto.decrypt(&ciphertext).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn should_fail_decrypt_with_wrong_passphrase() {
        let crypto1 = LocalCrypto::from_passphrase("passphrase-one", "same-salt").unwrap();
        let crypto2 = LocalCrypto::from_passphrase("passphrase-two", "same-salt").unwrap();

        let ciphertext = crypto1.encrypt(b"secret data").unwrap();
        let result = crypto2.decrypt(&ciphertext);

        assert!(matches!(result, Err(CryptoError::DecryptionFailed)));
    }

    #[test]
    fn should_fail_decrypt_with_wrong_salt() {
        let crypto1 = LocalCrypto::from_passphrase("same-passphrase", "salt-one").unwrap();
        let crypto2 = LocalCrypto::from_passphrase("same-passphrase", "salt-two").unwrap();

        let ciphertext = crypto1.encrypt(b"secret data").unwrap();
        let result = crypto2.decrypt(&ciphertext);

        assert!(matches!(result, Err(CryptoError::DecryptionFailed)));
    }

    #[test]
    fn should_fail_decrypt_when_ciphertext_is_tampered() {
        let crypto = test_crypto();
        let mut ciphertext = crypto.encrypt(b"secret data").unwrap();

        // Flip a bit in the ciphertext (after the nonce)
        ciphertext[NONCE_SIZE + 1] ^= 0x01;

        let result = crypto.decrypt(&ciphertext);

        assert!(matches!(result, Err(CryptoError::DecryptionFailed)));
    }

    #[test]
    fn should_fail_decrypt_when_ciphertext_is_truncated() {
        let crypto = test_crypto();

        // Less than NONCE_SIZE (12) + auth tag (16) = 28 bytes
        let short_data = vec![0u8; 27];
        let result = crypto.decrypt(&short_data);

        assert!(matches!(result, Err(CryptoError::InvalidCipherText)));
    }

    #[test]
    fn should_decrypt_across_instances_with_same_passphrase_and_salt() {
        let crypto1 = LocalCrypto::from_passphrase("shared-pass", "shared-salt").unwrap();
        let crypto2 = LocalCrypto::from_passphrase("shared-pass", "shared-salt").unwrap();

        let ciphertext = crypto1.encrypt(b"cross-instance secret").unwrap();
        let decrypted = crypto2.decrypt(&ciphertext).unwrap();

        assert_eq!(decrypted, b"cross-instance secret");
    }

    #[test]
    fn should_handle_empty_plaintext() {
        let crypto = test_crypto();

        let ciphertext = crypto.encrypt(b"").unwrap();
        let decrypted = crypto.decrypt(&ciphertext).unwrap();

        assert_eq!(decrypted, b"");
    }

    #[test]
    fn should_handle_large_plaintext() {
        let crypto = test_crypto();
        let large_plaintext = vec![0xAB; 1024 * 1024]; // 1MB

        let ciphertext = crypto.encrypt(&large_plaintext).unwrap();
        let decrypted = crypto.decrypt(&ciphertext).unwrap();

        assert_eq!(decrypted, large_plaintext);
    }

    #[test]
    fn should_produce_different_ciphertext_for_same_plaintext() {
        let crypto = test_crypto();
        let plaintext = b"same message";

        let ciphertext1 = crypto.encrypt(plaintext).unwrap();
        let ciphertext2 = crypto.encrypt(plaintext).unwrap();

        // Different nonces should produce different ciphertext
        assert_ne!(ciphertext1, ciphertext2);

        // But both should decrypt to the same plaintext
        assert_eq!(crypto.decrypt(&ciphertext1).unwrap(), plaintext);
        assert_eq!(crypto.decrypt(&ciphertext2).unwrap(), plaintext);
    }
}
