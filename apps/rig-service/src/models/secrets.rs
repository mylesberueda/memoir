use crate::api::crypto::get_crypto;
use common_rs::crypto::{CryptoError, Secret};
use sea_orm::{ActiveModelBehavior, ActiveValue, ConnectionTrait, DbErr};

pub(crate) use super::_entity::secrets::*;

const MARKER: &[u8] = "🔑x0".as_bytes();

#[derive(Debug, Clone, Copy, strum::EnumString, strum::Display, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum SecretKind {
    ApiKey,
    Token,
    EnvVar,
}

impl From<SecretKind> for sea_orm::Value {
    fn from(kind: SecretKind) -> Self {
        sea_orm::Value::String(Some(kind.to_string()))
    }
}

impl Model {
    pub fn decrypt(&self) -> Result<Secret, CryptoError> {
        get_crypto().decrypt_to_secret(
            self.encrypted_value
                .strip_prefix(MARKER)
                .ok_or(CryptoError::InvalidCipherText)?,
        )
    }
}

impl ModelEx {
    pub fn decrypt(&self) -> Result<Secret, CryptoError> {
        Model::from(self.to_owned()).decrypt()
    }
}

/// If there's a duplicate impl error, it's because the gen:entities command
/// added an implementation to the _entities/secrets.rs file. Delete the impl
/// in that file to fix.
#[tonic::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, sea_orm::DbErr>
    where
        C: ConnectionTrait,
    {
        if let ActiveValue::Set(ref plaintext) = self.encrypted_value
            && !plaintext.starts_with(MARKER)
        {
            let ciphertext: Vec<u8> = MARKER
                .iter()
                .copied()
                .chain(
                    get_crypto()
                        .encrypt(plaintext)
                        .map_err(|e| DbErr::Custom(format!("encryption failed: {e}")))?,
                )
                .collect();

            self.encrypted_value = ActiveValue::Set(ciphertext);
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    mod secret_kind {
        use super::*;

        #[test]
        fn should_serialize_api_key_to_kebab_case() {
            assert_eq!(SecretKind::ApiKey.to_string(), "api-key");
        }

        #[test]
        fn should_serialize_token_to_kebab_case() {
            assert_eq!(SecretKind::Token.to_string(), "token");
        }

        #[test]
        fn should_serialize_env_var_to_kebab_case() {
            assert_eq!(SecretKind::EnvVar.to_string(), "env-var");
        }

        #[test]
        fn should_parse_api_key_from_kebab_case() {
            let kind = SecretKind::from_str("api-key").unwrap();
            assert!(matches!(kind, SecretKind::ApiKey));
        }

        #[test]
        fn should_parse_token_from_kebab_case() {
            let kind = SecretKind::from_str("token").unwrap();
            assert!(matches!(kind, SecretKind::Token));
        }

        #[test]
        fn should_parse_env_var_from_kebab_case() {
            let kind = SecretKind::from_str("env-var").unwrap();
            assert!(matches!(kind, SecretKind::EnvVar));
        }

        #[test]
        fn should_fail_to_parse_invalid_string() {
            let result = SecretKind::from_str("invalid-kind");
            assert!(result.is_err());
        }

        #[test]
        fn should_convert_to_sea_orm_value() {
            let value: sea_orm::Value = SecretKind::ApiKey.into();
            assert_eq!(value, sea_orm::Value::String(Some("api-key".to_string())));
        }
    }
}

#[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::*;
    use crate::api::crypto;
    use common_rs::crypto::LocalCrypto;
    use sea_orm::{ActiveModelTrait, ActiveValue::Set, Database, DatabaseConnection, EntityTrait};
    use std::sync::Arc;

    async fn setup_db() -> Arc<DatabaseConnection> {
        let db_url = std::env::var("DATABASE_URL").expect("integration tests require DATABASE_URL");
        Arc::new(
            Database::connect(&db_url)
                .await
                .expect("database should be running for integration tests"),
        )
    }

    fn init_test_crypto() {
        // Use a fixed passphrase/salt for deterministic tests
        let crypto =
            LocalCrypto::from_passphrase("test-passphrase", "test-salt").expect("static test credentials are valid");
        crypto::init(crypto);
    }

    mod before_save {
        use super::*;

        #[tokio::test]
        async fn should_encrypt_plaintext_and_prepend_marker_on_insert() {
            init_test_crypto();
            let db = setup_db().await;
            let unique = nanoid::nanoid!();

            let plaintext = b"sk-test-api-key-12345";
            let secret = ActiveModel {
                pid: Set(format!("secret_{unique}")),
                secret_type: Set(SecretKind::ApiKey.to_string()),
                encrypted_value: Set(plaintext.to_vec()),
                ..Default::default()
            };

            let saved = secret
                .insert(db.as_ref())
                .await
                .expect("secrets table exists and accepts inserts");

            // Verify marker is prepended
            assert!(
                saved.encrypted_value.starts_with(MARKER),
                "encrypted_value should start with marker, got: {:?}",
                &saved.encrypted_value[..MARKER.len().min(saved.encrypted_value.len())]
            );

            // Verify it's not plaintext anymore
            assert_ne!(saved.encrypted_value, plaintext);

            // Verify length increased (marker + nonce + ciphertext + auth tag)
            assert!(
                saved.encrypted_value.len() > plaintext.len() + MARKER.len(),
                "encrypted value should be longer than plaintext + marker"
            );
        }

        #[tokio::test]
        async fn should_not_double_encrypt_when_marker_already_present() {
            init_test_crypto();
            let db = setup_db().await;
            let unique = nanoid::nanoid!();

            // First insert - will encrypt
            let plaintext = b"sk-original-key";
            let secret = ActiveModel {
                pid: Set(format!("secret_{unique}")),
                secret_type: Set(SecretKind::ApiKey.to_string()),
                encrypted_value: Set(plaintext.to_vec()),
                ..Default::default()
            };

            let saved = secret
                .insert(db.as_ref())
                .await
                .expect("secrets table exists and accepts inserts");
            let first_encrypted = saved.encrypted_value.clone();

            // Update with the already-encrypted value (simulating a re-save)
            let mut update: ActiveModel = saved.into();
            update.encrypted_value = Set(first_encrypted.clone());

            let updated = update
                .update(db.as_ref())
                .await
                .expect("update should succeed on existing row");

            // Should be identical - no double encryption
            assert_eq!(
                updated.encrypted_value, first_encrypted,
                "should not double-encrypt when marker is present"
            );
        }

        /// Documents that pre-encrypting without marker causes double encryption.
        /// This is an anti-pattern - always pass plaintext and let before_save handle it.
        #[tokio::test]
        async fn should_cause_double_encryption_when_pre_encrypted_without_marker() {
            use common_rs::crypto::SecretCrypto as _;

            init_test_crypto();
            let db = setup_db().await;
            let unique = nanoid::nanoid!();

            let plaintext = "sk-test-api-key-12345";

            // ANTI-PATTERN: pre-encrypting without marker causes double encryption
            let pre_encrypted = get_crypto().encrypt(plaintext.as_bytes()).unwrap();
            assert!(!pre_encrypted.starts_with(MARKER));

            let secret = ActiveModel {
                pid: Set(format!("secret_{unique}")),
                secret_type: Set(SecretKind::ApiKey.to_string()),
                encrypted_value: Set(pre_encrypted),
                ..Default::default()
            };

            let saved = secret.insert(db.as_ref()).await.expect("insert should succeed");

            // before_save encrypts again (no marker) = double encryption = decryption fails
            assert!(
                saved.decrypt().is_err(),
                "double encryption should cause decryption failure"
            );
        }
    }

    mod decrypt {
        use super::*;

        #[tokio::test]
        async fn should_decrypt_to_original_plaintext() {
            init_test_crypto();
            let db = setup_db().await;
            let unique = nanoid::nanoid!();

            let plaintext = "sk-my-secret-api-key-67890";
            let secret = ActiveModel {
                pid: Set(format!("secret_{unique}")),
                secret_type: Set(SecretKind::ApiKey.to_string()),
                encrypted_value: Set(plaintext.as_bytes().to_vec()),
                ..Default::default()
            };

            let saved = secret
                .insert(db.as_ref())
                .await
                .expect("secrets table exists and accepts inserts");

            let decrypted = saved.decrypt().expect("crypto is initialized and data is valid");

            assert_eq!(decrypted.expose(), plaintext);
        }

        #[tokio::test]
        async fn should_fail_when_marker_is_missing() {
            init_test_crypto();
            let db = setup_db().await;

            // Create a model with raw data (no marker) - simulating corrupt/legacy data
            let model = Model {
                id: 0,
                pid: "test".into(),
                secret_type: "api-key".into(),
                encrypted_value: b"raw-data-without-marker".to_vec(),
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: chrono::Utc::now().naive_utc(),
            };

            let result = model.decrypt();

            assert!(
                matches!(result, Err(CryptoError::InvalidCipherText)),
                "should fail with InvalidCipherText when marker is missing"
            );
        }

        #[tokio::test]
        async fn should_roundtrip_empty_secret() {
            init_test_crypto();
            let db = setup_db().await;
            let unique = nanoid::nanoid!();

            let plaintext = "";
            let secret = ActiveModel {
                pid: Set(format!("secret_{unique}")),
                secret_type: Set(SecretKind::Token.to_string()),
                encrypted_value: Set(plaintext.as_bytes().to_vec()),
                ..Default::default()
            };

            let saved = secret
                .insert(db.as_ref())
                .await
                .expect("secrets table exists and accepts inserts");
            let decrypted = saved.decrypt().expect("crypto is initialized and data is valid");

            assert_eq!(decrypted.expose(), plaintext);
        }

        #[tokio::test]
        async fn should_roundtrip_unicode_secret() {
            init_test_crypto();
            let db = setup_db().await;
            let unique = nanoid::nanoid!();

            let plaintext = "密钥🔐émojis-and-ünïcödé";
            let secret = ActiveModel {
                pid: Set(format!("secret_{unique}")),
                secret_type: Set(SecretKind::EnvVar.to_string()),
                encrypted_value: Set(plaintext.as_bytes().to_vec()),
                ..Default::default()
            };

            let saved = secret
                .insert(db.as_ref())
                .await
                .expect("secrets table exists and accepts inserts");
            let decrypted = saved.decrypt().expect("crypto is initialized and data is valid");

            assert_eq!(decrypted.expose(), plaintext);
        }
    }
}
