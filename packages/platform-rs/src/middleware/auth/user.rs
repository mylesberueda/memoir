use std::collections::HashMap;

/// Unified user representation extracted from authentication tokens
#[derive(Debug, Clone)]
pub struct User {
    /// Unique user identifier. We use `sub` right now, but be wary of using it
    /// when setting up your own. Most providers don't allow you to set `sub`,
    /// which makes auth provider migrations terrible.
    pub id: String,
    /// User's email address
    pub email: Option<String>,
    /// Display name
    pub name: Option<String>,
    /// Roles by organization
    pub org_roles: HashMap<String, Vec<String>>,
    /// If the user's email is verified
    pub email_verified: Option<bool>,
    /// Additional key-value metadata from the identity provider
    pub metadata: HashMap<String, String>,
}

impl User {
    /// Get all roles the user has in a specific organization
    pub fn roles_in(&self, organization_pid: &str) -> &[String] {
        self.org_roles
            .get(organization_pid)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Check if user has a specific role in an organization
    pub fn has_role_in(&self, organization_id: &str, role: &str) -> bool {
        self.org_roles
            .get(organization_id)
            .map(|roles| roles.iter().any(|r| r == role))
            .unwrap_or(false)
    }

    /// Check if the user is a member of an organization
    pub fn is_member_of(&self, organization_id: &str) -> bool {
        self.org_roles.contains_key(organization_id)
    }

    /// Get all organization ids the user belongs to
    pub fn organizations(&self) -> impl Iterator<Item = &str> {
        self.org_roles.keys().map(|s| s.as_str())
    }

    /// Check if the user has a role in ANY organization
    pub fn has_role_anywhere(&self, role: &str) -> bool {
        self.org_roles.values().any(|roles| roles.iter().any(|r| r == role))
    }

    /// Get a metadata value by key
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }
}

/// Trait for extracting a `User` from validated token claims.
/// Each identity provider implements this to handle their specific claim formats.
pub trait UserExtractor: Default + Send + Sync + 'static {
    /// Extract user data from validated token claims
    fn extract_user(
        &self,
        access_token_claims: &serde_json::Value,
        id_token_claims: Option<&serde_json::Value>,
    ) -> crate::Result<User>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_user() -> User {
        let mut org_roles = HashMap::new();
        org_roles.insert("org_acme".to_string(), vec!["admin".to_string(), "finance".to_string()]);
        org_roles.insert("org_other".to_string(), vec!["viewer".to_string()]);

        let mut metadata = HashMap::new();
        metadata.insert("tier".to_string(), "plus".to_string());
        metadata.insert("locale".to_string(), "en-US".to_string());

        User {
            id: "user-123".to_string(),
            email: Some("test@example.com".to_string()),
            name: Some("Test User".to_string()),
            org_roles,
            email_verified: Some(true),
            metadata,
        }
    }

    #[test]
    fn should_return_roles_for_known_org() {
        let user = test_user();

        let roles = user.roles_in("org_acme");

        assert_eq!(roles.len(), 2);
        assert!(roles.contains(&"admin".to_string()));
        assert!(roles.contains(&"finance".to_string()));
    }

    #[test]
    fn should_return_empty_slice_for_unknown_org() {
        let user = test_user();

        let roles = user.roles_in("org_nonexistent");

        assert!(roles.is_empty());
    }

    #[test]
    fn should_check_role_in_org() {
        let user = test_user();

        assert!(user.has_role_in("org_acme", "admin"));
        assert!(user.has_role_in("org_acme", "finance"));
        assert!(!user.has_role_in("org_acme", "viewer"));
        assert!(!user.has_role_in("org_acme", "superadmin"));
    }

    #[test]
    fn should_return_false_for_role_in_unknown_org() {
        let user = test_user();

        assert!(!user.has_role_in("org_nonexistent", "admin"));
    }

    #[test]
    fn should_check_org_membership() {
        let user = test_user();

        assert!(user.is_member_of("org_acme"));
        assert!(user.is_member_of("org_other"));
        assert!(!user.is_member_of("org_unknown"));
    }

    #[test]
    fn should_list_all_organizations() {
        let user = test_user();

        let orgs: Vec<&str> = user.organizations().collect();

        assert_eq!(orgs.len(), 2);
        assert!(orgs.contains(&"org_acme"));
        assert!(orgs.contains(&"org_other"));
    }

    #[test]
    fn should_check_role_anywhere() {
        let user = test_user();

        assert!(user.has_role_anywhere("admin"));
        assert!(user.has_role_anywhere("viewer"));
        assert!(!user.has_role_anywhere("superadmin"));
    }

    #[test]
    fn should_get_metadata_value() {
        let user = test_user();

        assert_eq!(user.get_metadata("tier"), Some("plus"));
        assert_eq!(user.get_metadata("locale"), Some("en-US"));
        assert_eq!(user.get_metadata("missing"), None);
    }
}
