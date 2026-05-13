use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Resource types that the permission matrix governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, strum::EnumIter, strum::Display)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum ResourceType {
    Agents,
    Conversations,
    Documents,
    Channels,
    Billing,
    Members,
}

/// Read/write/execute permission flags for a single resource type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourcePermission {
    #[serde(default)]
    pub read: bool,
    #[serde(default)]
    pub write: bool,
    #[serde(default)]
    pub execute: bool,
}

impl ResourcePermission {
    pub const NONE: Self = Self { read: false, write: false, execute: false };
    pub const READ: Self = Self { read: true, write: false, execute: false };
    pub const RW: Self = Self { read: true, write: true, execute: false };
    pub const RX: Self = Self { read: true, write: false, execute: true };
    pub const RWX: Self = Self { read: true, write: true, execute: true };
}

/// Pre-resolved permission matrix for a user's org membership.
///
/// Computed by api-service from role defaults + per-member overrides.
/// Cached in `CachedOrg` so consumer services do a single field lookup.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedPermissions(HashMap<ResourceType, ResourcePermission>);

impl ResolvedPermissions {
    pub fn new(map: HashMap<ResourceType, ResourcePermission>) -> Self {
        Self(map)
    }

    /// Permissions that grant full access to all resource types.
    /// Used as a fallback when OrgContext is absent (backwards compat).
    pub fn allow_all() -> Self {
        use strum::IntoEnumIterator as _;
        Self(ResourceType::iter().map(|r| (r, ResourcePermission::RWX)).collect())
    }

    /// Check if a specific permission is granted for a resource type.
    pub fn can_read(&self, resource: ResourceType) -> bool {
        self.0.get(&resource).is_some_and(|p| p.read)
    }

    pub fn can_write(&self, resource: ResourceType) -> bool {
        self.0.get(&resource).is_some_and(|p| p.write)
    }

    pub fn can_execute(&self, resource: ResourceType) -> bool {
        self.0.get(&resource).is_some_and(|p| p.execute)
    }

    /// Get the full permission for a resource type.
    pub fn get(&self, resource: ResourceType) -> ResourcePermission {
        self.0.get(&resource).copied().unwrap_or(ResourcePermission::NONE)
    }
}

impl Default for ResolvedPermissions {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_check_individual_permissions() {
        let mut map = HashMap::new();
        map.insert(ResourceType::Agents, ResourcePermission::RX);
        map.insert(ResourceType::Billing, ResourcePermission::NONE);
        let perms = ResolvedPermissions::new(map);

        assert!(perms.can_read(ResourceType::Agents));
        assert!(!perms.can_write(ResourceType::Agents));
        assert!(perms.can_execute(ResourceType::Agents));
        assert!(!perms.can_read(ResourceType::Billing));
    }

    #[test]
    fn should_return_none_for_missing_resource() {
        let perms = ResolvedPermissions::default();

        assert!(!perms.can_read(ResourceType::Agents));
        assert_eq!(perms.get(ResourceType::Agents), ResourcePermission::NONE);
    }

    #[test]
    fn should_serialize_roundtrip() {
        let mut map = HashMap::new();
        map.insert(ResourceType::Agents, ResourcePermission::RWX);
        map.insert(ResourceType::Billing, ResourcePermission::READ);
        let perms = ResolvedPermissions::new(map);

        let json = serde_json::to_string(&perms).unwrap();
        let deserialized: ResolvedPermissions = serde_json::from_str(&json).unwrap();
        assert_eq!(perms, deserialized);
    }

    #[test]
    fn should_serialize_resource_type_as_kebab_case() {
        assert_eq!(serde_json::to_string(&ResourceType::Agents).unwrap(), "\"agents\"");
        assert_eq!(serde_json::to_string(&ResourceType::Billing).unwrap(), "\"billing\"");
    }
}
