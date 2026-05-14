//! Password hashing and API key generation for Memoir auth storage.
//!
//! Provides Argon2id PHC-format password hashing with constant-time verification,
//! and CSPRNG-backed API key generation in the canonical Memoir format
//! `mk.<key_id>.<secret>`.
//!
//! # Storage model
//!
//! Both passwords and API key secrets are stored as Argon2 PHC strings
//! (`$argon2id$v=19$m=...$<salt>$<hash>`) in the database. The PHC format
//! embeds the algorithm, parameters, and salt alongside the hash, so future
//! Argon2 tuning can be applied without invalidating existing hashes — the
//! verifier reads the parameters from the stored string.
//!
//! # API key format
//!
//! Keys have the shape `mk.<key_id>.<secret>` where:
//!
//! - `mk` is a fixed prefix identifying a Memoir key.
//! - `<key_id>` is a short non-secret identifier (8 random bytes,
//!   base64url-encoded). It is also stored as the unique lookup column on
//!   `api_keys.key_id` so the auth interceptor can find the matching row
//!   in O(log n) time.
//! - `<secret>` is the secret half (32 random bytes, base64url-encoded).
//!   Argon2-hashed at rest in `api_keys.key_hash`.
//!
//! On verification, [`parse_api_key`] splits the input and the caller looks
//! up the row by `key_id`, then calls [`verify_password`] with the secret
//! half against the stored hash.
//!
//! # Security
//!
//! - Salts are randomly generated per hash via the PHC API; never reused.
//! - Verification uses [`argon2::PasswordVerifier::verify_password`], which
//!   is constant-time to prevent timing side-channels.
//! - Random material (salts, key bytes) comes from [`rand_core::OsRng`],
//!   a cryptographically secure source backed by the operating system.

use argon2::password_hash::PasswordHasher;
use argon2::{Argon2, PasswordVerifier};
use base64::Engine as _;
use rand_core::{OsRng, RngCore as _};

use super::CryptoError;

/// Prefix identifying a Memoir API key on the wire.
///
/// Stable contract — clients and the auth interceptor both depend on this
/// exact value to recognize and parse keys.
const KEY_PREFIX: &str = "mk";

/// Length in bytes of the random `key_id` portion of an API key.
///
/// 8 bytes = 64 bits of entropy. Encoded as ~12 base64url characters.
/// At 10M issued keys the birthday-collision probability is < 2^-32,
/// which is acceptable for a non-secret lookup index where the secret
/// half provides actual authentication.
const KEY_ID_BYTES: usize = 8;

/// Length in bytes of the random secret portion of an API key.
///
/// 32 bytes = 256 bits of entropy. Encoded as ~43 base64url characters.
/// Far above any practical brute-force threshold; matches OWASP guidance
/// for opaque session tokens.
const SECRET_BYTES: usize = 32;

/// A freshly generated API key with its plaintext, key_id, and secret split out.
///
/// The plaintext field is the full wire representation (`mk.<key_id>.<secret>`)
/// that callers should hand to the user exactly once. The `key_id` is stored
/// in the database as the lookup index; the `secret` is hashed via
/// [`hash_password`] and stored as `key_hash`.
#[derive(Debug)]
pub struct ApiKey {
    /// Full wire format: `mk.<key_id>.<secret>`. Display this to the caller once.
    pub plaintext: String,

    /// Non-secret lookup half. Persist as `api_keys.key_id`.
    pub key_id: String,

    /// Secret half. Hash via [`hash_password`] and persist as `api_keys.key_hash`.
    pub secret: String,
}

/// Hashes `password` using Argon2id with a randomly generated salt.
///
/// Returns a PHC-format string suitable for direct database storage.
///
/// # Examples
///
/// ```
/// use common_rs::crypto::hashing::{hash_password, verify_password};
///
/// let phc = hash_password("hunter2").unwrap();
/// assert!(verify_password("hunter2", &phc).unwrap());
/// assert!(!verify_password("wrong", &phc).unwrap());
/// ```
///
/// # Errors
///
/// Returns [`CryptoError::HashFailed`] if Argon2 fails to derive a hash,
/// which in practice indicates a misconfigured Argon2 parameter set or
/// a corrupted system RNG.
pub fn hash_password(password: impl AsRef<[u8]>) -> Result<String, CryptoError> {
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_ref())
        .map_err(|_| CryptoError::HashFailed)?;
    Ok(hash.to_string())
}

/// Verifies `password` against a stored Argon2 PHC hash in constant time.
///
/// Returns `Ok(true)` on match, `Ok(false)` on mismatch (verification ran
/// successfully and rejected the input). Returns [`CryptoError::VerifyFailed`]
/// only when the stored hash is malformed and verification could not run.
///
/// # Examples
///
/// ```
/// use common_rs::crypto::hashing::{hash_password, verify_password};
///
/// let phc = hash_password("correct").unwrap();
/// assert!(verify_password("correct", &phc).unwrap());
/// assert!(!verify_password("incorrect", &phc).unwrap());
/// ```
///
/// # Errors
///
/// Returns [`CryptoError::VerifyFailed`] when `stored_hash` cannot be parsed
/// as a PHC string (corrupted DB row, schema mismatch).
pub fn verify_password(
    password: impl AsRef<[u8]>,
    stored_hash: &str,
) -> Result<bool, CryptoError> {
    let parsed = argon2::password_hash::phc::PasswordHash::new(stored_hash)
        .map_err(|_| CryptoError::VerifyFailed)?;
    match Argon2::default().verify_password(password.as_ref(), &parsed) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::PasswordInvalid) => Ok(false),
        Err(_) => Err(CryptoError::VerifyFailed),
    }
}

/// Generates a new API key with a random key_id and secret.
///
/// The returned [`ApiKey`] carries the full plaintext (`mk.<key_id>.<secret>`),
/// the lookup index (`key_id`), and the secret to be hashed and stored.
///
/// # Examples
///
/// ```
/// use common_rs::crypto::hashing::{generate_api_key, parse_api_key};
///
/// let key = generate_api_key().unwrap();
/// assert!(key.plaintext.starts_with("mk_"));
///
/// let parsed = parse_api_key(&key.plaintext).unwrap();
/// assert_eq!(parsed.0, key.key_id);
/// assert_eq!(parsed.1, key.secret);
/// ```
///
/// # Errors
///
/// Returns [`CryptoError::RngFailed`] if the OS CSPRNG cannot produce
/// random bytes (an extraordinary failure mode that typically indicates
/// a misconfigured kernel entropy source).
pub fn generate_api_key() -> Result<ApiKey, CryptoError> {
    let mut id_bytes = [0u8; KEY_ID_BYTES];
    let mut secret_bytes = [0u8; SECRET_BYTES];
    OsRng
        .try_fill_bytes(&mut id_bytes)
        .map_err(|_| CryptoError::RngFailed)?;
    OsRng
        .try_fill_bytes(&mut secret_bytes)
        .map_err(|_| CryptoError::RngFailed)?;

    let key_id = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(id_bytes);
    let secret = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(secret_bytes);
    let plaintext = format!("{KEY_PREFIX}.{key_id}.{secret}");

    Ok(ApiKey {
        plaintext,
        key_id,
        secret,
    })
}

/// Parses an API key string of shape `mk.<key_id>.<secret>` into its parts.
///
/// Returns `(key_id, secret)` on success. Used by the auth interceptor to
/// split a bearer token presented in request metadata.
///
/// The `.` separator is chosen over `_` because the base64url alphabet for
/// the key_id and secret halves includes `_`, which would make an
/// underscore-separated format ambiguous to parse.
///
/// # Examples
///
/// ```
/// use common_rs::crypto::hashing::parse_api_key;
///
/// let (id, secret) = parse_api_key("mk.abc123.xyz789").unwrap();
/// assert_eq!(id, "abc123");
/// assert_eq!(secret, "xyz789");
/// ```
///
/// # Errors
///
/// Returns [`CryptoError::InvalidKeyFormat`] when the input does not start
/// with the `mk.` prefix, lacks the expected separator, or has empty parts.
pub fn parse_api_key(key: &str) -> Result<(&str, &str), CryptoError> {
    let rest = key
        .strip_prefix(KEY_PREFIX)
        .and_then(|s| s.strip_prefix('.'))
        .ok_or(CryptoError::InvalidKeyFormat)?;
    let (key_id, secret) = rest.split_once('.').ok_or(CryptoError::InvalidKeyFormat)?;
    if key_id.is_empty() || secret.is_empty() {
        return Err(CryptoError::InvalidKeyFormat);
    }
    Ok((key_id, secret))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_hash_and_verify_roundtrip() {
        let phc = hash_password("hunter2").unwrap();
        assert!(verify_password("hunter2", &phc).unwrap());
    }

    #[test]
    fn should_reject_wrong_password() {
        let phc = hash_password("correct").unwrap();
        assert!(!verify_password("wrong", &phc).unwrap());
    }

    #[test]
    fn should_produce_phc_format_hash() {
        let phc = hash_password("password").unwrap();
        assert!(phc.starts_with("$argon2"), "expected PHC format, got: {phc}");
    }

    #[test]
    fn should_use_different_salts_for_same_password() {
        let phc1 = hash_password("same").unwrap();
        let phc2 = hash_password("same").unwrap();
        assert_ne!(phc1, phc2, "salts must differ between hashes");
        assert!(verify_password("same", &phc1).unwrap());
        assert!(verify_password("same", &phc2).unwrap());
    }

    #[test]
    fn should_return_verify_failed_for_malformed_hash() {
        let result = verify_password("anything", "not-a-phc-string");
        assert!(matches!(result, Err(CryptoError::VerifyFailed)));
    }

    #[test]
    fn should_generate_api_key_with_mk_prefix() {
        let key = generate_api_key().unwrap();
        assert!(key.plaintext.starts_with("mk."));
    }

    #[test]
    fn should_generate_api_key_with_correct_part_lengths() {
        let key = generate_api_key().unwrap();
        // 8 bytes base64url-no-pad → 11 chars; 32 bytes → 43 chars
        assert_eq!(key.key_id.len(), 11);
        assert_eq!(key.secret.len(), 43);
    }

    #[test]
    fn should_generate_unique_api_keys() {
        let k1 = generate_api_key().unwrap();
        let k2 = generate_api_key().unwrap();
        assert_ne!(k1.plaintext, k2.plaintext);
        assert_ne!(k1.key_id, k2.key_id);
        assert_ne!(k1.secret, k2.secret);
    }

    #[test]
    fn should_parse_generated_api_key() {
        let key = generate_api_key().unwrap();
        let (id, secret) = parse_api_key(&key.plaintext).unwrap();
        assert_eq!(id, key.key_id);
        assert_eq!(secret, key.secret);
    }

    #[test]
    fn should_reject_key_without_mk_prefix() {
        let result = parse_api_key("nope.abc.xyz");
        assert!(matches!(result, Err(CryptoError::InvalidKeyFormat)));
    }

    #[test]
    fn should_reject_key_without_separator() {
        let result = parse_api_key("mk.abcxyz");
        assert!(matches!(result, Err(CryptoError::InvalidKeyFormat)));
    }

    #[test]
    fn should_reject_key_with_empty_parts() {
        assert!(matches!(parse_api_key("mk..xyz"), Err(CryptoError::InvalidKeyFormat)));
        assert!(matches!(parse_api_key("mk.abc."), Err(CryptoError::InvalidKeyFormat)));
    }

    #[test]
    fn should_reject_empty_input() {
        assert!(matches!(parse_api_key(""), Err(CryptoError::InvalidKeyFormat)));
    }

    #[test]
    fn should_verify_generated_api_key_secret() {
        // End-to-end: generate, hash secret, verify the secret half against the hash.
        let key = generate_api_key().unwrap();
        let stored_hash = hash_password(&key.secret).unwrap();
        let (_id, secret) = parse_api_key(&key.plaintext).unwrap();
        assert!(verify_password(secret, &stored_hash).unwrap());
    }
}
