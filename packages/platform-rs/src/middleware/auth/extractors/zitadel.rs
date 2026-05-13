use std::collections::HashMap;

use base64::{Engine as _, prelude::BASE64_STANDARD_NO_PAD};

use crate::middleware::auth::{User, UserExtractor};

mod claims {
    pub const ROLES: &str = "urn:zitadel:iam:org:project:roles";
    pub const METADATA: &str = "urn:zitadel:iam:user:metadata";
}

#[derive(Debug, Clone, Default)]
pub struct ZitadelUserExtractor;

impl ZitadelUserExtractor {
    pub fn new() -> Self {
        Self
    }

    /// Extract roles grouped by organization
    fn extract_org_roles(claims: &serde_json::Value) -> HashMap<String, Vec<String>> {
        let mut org_roles: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(roles_json) = claims.get(claims::ROLES).and_then(|v| v.as_object()) {
            for (role, orgs) in roles_json {
                if let Some(orgs_json) = orgs.as_object() {
                    for org_id in orgs_json.keys() {
                        org_roles.entry(org_id.clone()).or_default().push(role.clone());
                    }
                }
            }
        }

        org_roles
    }

    /// Extract and decode metadata from Zitadel's metadata claim
    fn extract_metadata(claims: &serde_json::Value) -> HashMap<String, String> {
        claims
            .get(claims::METADATA)
            .and_then(|v| v.as_object())
            .map(|o| {
                o.iter()
                    .filter_map(|(k, v)| {
                        let encoded = v.as_str()?;
                        let bytes = BASE64_STANDARD_NO_PAD.decode(encoded).ok()?;
                        let decoded = String::from_utf8(bytes).ok()?;

                        Some((k.clone(), decoded))
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get a string claim, trying the identity token first, then the access token
    fn get_claim<'a>(
        access_token: &'a serde_json::Value,
        identity_token: Option<&'a serde_json::Value>,
        key: &str,
    ) -> Option<&'a str> {
        identity_token
            .and_then(|t| t.get(key))
            .or_else(|| access_token.get(key))
            .and_then(|v| v.as_str())
    }

    /// Get a boolean claim, trying the identity token first, then the access token
    fn get_bool_claim(
        access_token: &serde_json::Value,
        identity_token: Option<&serde_json::Value>,
        key: &str,
    ) -> Option<bool> {
        identity_token
            .and_then(|t| t.get(key))
            .or_else(|| access_token.get(key))
            .and_then(|v| v.as_bool())
    }
}

impl UserExtractor for ZitadelUserExtractor {
    fn extract_user(
        &self,
        access_token_claims: &serde_json::Value,
        identity_token_claims: Option<&serde_json::Value>,
    ) -> crate::Result<User> {
        let id = access_token_claims
            .get("sub")
            .and_then(|v| v.as_str())
            .ok_or_else(|| color_eyre::eyre::eyre!("Missing 'sub' claim in access token"))?
            .to_string();

        let org_roles = identity_token_claims
            .map(Self::extract_org_roles)
            .unwrap_or_else(|| Self::extract_org_roles(access_token_claims));

        let metadata = identity_token_claims
            .map(Self::extract_metadata)
            .unwrap_or_else(|| Self::extract_metadata(access_token_claims));

        let email = Self::get_claim(access_token_claims, identity_token_claims, "email").map(String::from);

        let email_verified = Self::get_bool_claim(access_token_claims, identity_token_claims, "email_verified");

        let name = Self::get_claim(access_token_claims, identity_token_claims, "name")
            .or_else(|| Self::get_claim(access_token_claims, identity_token_claims, "preferred_username"))
            .or_else(|| Self::get_claim(access_token_claims, identity_token_claims, "email"))
            .map(String::from);

        Ok(User {
            id,
            email,
            name,
            org_roles,
            email_verified,
            metadata,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn should_extract_full_user_from_both_tokens() {
        let extractor = ZitadelUserExtractor;

        let access = json!({ "sub": "user-123" });
        let id_token = json!({
            "sub": "user-123",
            "email": "test@example.com",
            "email_verified": true,
            "name": "Test User",
            "urn:zitadel:iam:org:project:roles": {
                "admin": { "org_acme": "acme.com" },
                "finance": { "org_acme": "acme.com" },
                "viewer": { "org_other": "other.com" }
            },
            "urn:zitadel:iam:user:metadata": {
                "tier": "cGx1cw"
            }
        });

        let user = extractor.extract_user(&access, Some(&id_token)).unwrap();

        assert_eq!(user.id, "user-123");
        assert_eq!(user.email, Some("test@example.com".to_string()));
        assert_eq!(user.name, Some("Test User".to_string()));
        assert_eq!(user.email_verified, Some(true));
        assert!(user.has_role_in("org_acme", "admin"));
        assert!(user.has_role_in("org_acme", "finance"));
        assert!(user.has_role_in("org_other", "viewer"));
        assert!(!user.has_role_in("org_other", "admin"));
    }

    #[test]
    fn should_extract_user_without_id_token() {
        let extractor = ZitadelUserExtractor;

        let access = json!({
            "sub": "user-456",
            "email": "access@example.com"
        });

        let user = extractor.extract_user(&access, None).unwrap();

        assert_eq!(user.id, "user-456");
        assert_eq!(user.email, Some("access@example.com".to_string()));
        // Name falls back to email when no name or preferred_username
        assert_eq!(user.name, Some("access@example.com".to_string()));
        assert!(user.org_roles.is_empty());
        assert!(user.metadata.is_empty());
        assert_eq!(user.email_verified, None);
    }

    #[test]
    fn should_fail_when_sub_claim_missing() {
        let extractor = ZitadelUserExtractor;
        let access = json!({ "email": "no-sub@example.com" });

        let result = extractor.extract_user(&access, None);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("sub"));
    }

    #[test]
    fn should_transform_roles_to_org_centric() {
        let claims = json!({
            "urn:zitadel:iam:org:project:roles": {
                "admin": { "org_1": "one.com", "org_2": "two.com" },
                "viewer": { "org_1": "one.com" },
                "billing": { "org_2": "two.com" }
            }
        });

        let org_roles = ZitadelUserExtractor::extract_org_roles(&claims);

        let mut org1_roles = org_roles.get("org_1").unwrap().clone();
        org1_roles.sort();
        assert_eq!(org1_roles, vec!["admin", "viewer"]);

        let mut org2_roles = org_roles.get("org_2").unwrap().clone();
        org2_roles.sort();
        assert_eq!(org2_roles, vec!["admin", "billing"]);
    }

    #[test]
    fn should_handle_empty_roles_object() {
        let claims = json!({
            "urn:zitadel:iam:org:project:roles": {}
        });

        let org_roles = ZitadelUserExtractor::extract_org_roles(&claims);

        assert!(org_roles.is_empty());
    }

    #[test]
    fn should_handle_missing_roles_claim() {
        let claims = json!({ "sub": "user-123" });

        let org_roles = ZitadelUserExtractor::extract_org_roles(&claims);

        assert!(org_roles.is_empty());
    }

    #[test]
    fn should_decode_base64_metadata() {
        // "plus" base64-encoded with STANDARD_NO_PAD = "cGx1cw"
        let claims = json!({
            "urn:zitadel:iam:user:metadata": {
                "tier": "cGx1cw",
                "locale": "ZW4tVVM"
            }
        });

        let metadata = ZitadelUserExtractor::extract_metadata(&claims);

        assert_eq!(metadata.get("tier").unwrap(), "plus");
        assert_eq!(metadata.get("locale").unwrap(), "en-US");
    }

    #[test]
    fn should_skip_invalid_base64_metadata() {
        let claims = json!({
            "urn:zitadel:iam:user:metadata": {
                "good": "cGx1cw",
                "bad": "!!!not-base64!!!",
                "also_good": "ZW4tVVM"
            }
        });

        let metadata = ZitadelUserExtractor::extract_metadata(&claims);

        assert_eq!(metadata.len(), 2);
        assert_eq!(metadata.get("good").unwrap(), "plus");
        assert_eq!(metadata.get("also_good").unwrap(), "en-US");
        assert!(!metadata.contains_key("bad"));
    }

    #[test]
    fn should_handle_missing_metadata_claim() {
        let claims = json!({ "sub": "user-123" });

        let metadata = ZitadelUserExtractor::extract_metadata(&claims);

        assert!(metadata.is_empty());
    }

    #[test]
    fn should_prefer_id_token_claims_over_access_token() {
        let access = json!({
            "email": "access@example.com",
            "email_verified": false
        });
        let id_token = json!({
            "email": "id@example.com",
            "email_verified": true
        });

        let email = ZitadelUserExtractor::get_claim(&access, Some(&id_token), "email");
        let verified = ZitadelUserExtractor::get_bool_claim(&access, Some(&id_token), "email_verified");

        assert_eq!(email, Some("id@example.com"));
        assert_eq!(verified, Some(true));
    }

    #[test]
    fn should_fallback_to_access_token_when_id_token_lacks_claim() {
        let access = json!({ "email": "access@example.com" });
        let id_token = json!({ "name": "Test User" });

        let email = ZitadelUserExtractor::get_claim(&access, Some(&id_token), "email");

        assert_eq!(email, Some("access@example.com"));
    }

    #[test]
    fn should_fallback_name_to_preferred_username() {
        let extractor = ZitadelUserExtractor;

        let access = json!({
            "sub": "user-789",
            "preferred_username": "jdoe"
        });

        let user = extractor.extract_user(&access, None).unwrap();

        assert_eq!(user.name, Some("jdoe".to_string()));
    }

    #[test]
    fn should_fallback_name_to_email() {
        let extractor = ZitadelUserExtractor;

        let access = json!({
            "sub": "user-789",
            "email": "jdoe@example.com"
        });

        let user = extractor.extract_user(&access, None).unwrap();

        assert_eq!(user.name, Some("jdoe@example.com".to_string()));
    }

    #[test]
    fn should_return_none_name_when_no_fallbacks() {
        let extractor = ZitadelUserExtractor;

        let access = json!({ "sub": "user-789" });

        let user = extractor.extract_user(&access, None).unwrap();

        assert_eq!(user.name, None);
    }
}
