use std::collections::HashMap;

use platform_rs::cache::{OrgRole, ResolvedPermissions, ResourcePermission, ResourceType};
use strum::IntoEnumIterator as _;

/// Resolve the effective permissions for a member given their role and any JSON overrides.
///
/// The override map uses the same `ResourceType` keys. When present, individual
/// fields (read/write/execute) override the role default for that resource.
/// Absent fields in the override fall back to the role default.
pub(crate) fn resolve_permissions(
    role: OrgRole,
    overrides: &HashMap<ResourceType, ResourcePermission>,
) -> ResolvedPermissions {
    let defaults = role_defaults(role);
    let mut map = HashMap::new();

    for resource in ResourceType::iter() {
        let base = defaults.get(&resource).copied().unwrap_or(ResourcePermission::NONE);
        let effective = match overrides.get(&resource) {
            Some(ovr) => ResourcePermission {
                read: ovr.read || base.read,
                write: ovr.write || base.write,
                execute: ovr.execute || base.execute,
            },
            None => base,
        };
        map.insert(resource, effective);
    }

    ResolvedPermissions::new(map)
}

/// Default permission matrix per role.
fn role_defaults(role: OrgRole) -> HashMap<ResourceType, ResourcePermission> {
    use ResourcePermission as P;
    use ResourceType::*;

    let pairs: &[(ResourceType, ResourcePermission)] = match role {
        OrgRole::Owner => &[
            (Agents, P::RWX),
            (Conversations, P::RWX),
            (Documents, P::RWX),
            (Channels, P::RWX),
            (Billing, P::RWX),
            (Members, P::RWX),
        ],
        OrgRole::Admin => &[
            (Agents, P::RWX),
            (Conversations, P::RWX),
            (Documents, P::RWX),
            (Channels, P::RWX),
            (Billing, P::READ),
            (Members, P::RW),
        ],
        OrgRole::Member => &[
            (Agents, P::RX),
            (Conversations, P::RWX),
            (Documents, P::READ),
            (Channels, P::RWX),
            (Billing, P::NONE),
            (Members, P::READ),
        ],
        OrgRole::Guest => &[
            (Agents, P::READ),
            (Conversations, P::READ),
            (Documents, P::READ),
            (Channels, P::READ),
            (Billing, P::NONE),
            (Members, P::NONE),
        ],
    };

    pairs.iter().copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_full_access_for_owner() {
        let perms = resolve_permissions(OrgRole::Owner, &HashMap::new());

        assert!(perms.can_read(ResourceType::Agents));
        assert!(perms.can_write(ResourceType::Agents));
        assert!(perms.can_execute(ResourceType::Agents));
        assert!(perms.can_read(ResourceType::Billing));
        assert!(perms.can_write(ResourceType::Billing));
    }

    #[test]
    fn should_deny_billing_for_member() {
        let perms = resolve_permissions(OrgRole::Member, &HashMap::new());

        assert!(!perms.can_read(ResourceType::Billing));
        assert!(!perms.can_write(ResourceType::Billing));
    }

    #[test]
    fn should_deny_members_for_guest() {
        let perms = resolve_permissions(OrgRole::Guest, &HashMap::new());

        assert!(!perms.can_read(ResourceType::Members));
    }

    #[test]
    fn should_grant_member_rx_on_agents() {
        let perms = resolve_permissions(OrgRole::Member, &HashMap::new());

        assert!(perms.can_read(ResourceType::Agents));
        assert!(!perms.can_write(ResourceType::Agents));
        assert!(perms.can_execute(ResourceType::Agents));
    }

    #[test]
    fn should_apply_override_additively() {
        let mut overrides = HashMap::new();
        overrides.insert(
            ResourceType::Agents,
            ResourcePermission {
                read: false,
                write: true,
                execute: false,
            },
        );

        // Member has r-x on agents by default; override adds write
        let perms = resolve_permissions(OrgRole::Member, &overrides);

        assert!(perms.can_read(ResourceType::Agents)); // from default
        assert!(perms.can_write(ResourceType::Agents)); // from override
        assert!(perms.can_execute(ResourceType::Agents)); // from default
    }

    #[test]
    fn should_not_remove_default_permissions_via_override() {
        let mut overrides = HashMap::new();
        // Override with read=false — should NOT remove the default read
        overrides.insert(ResourceType::Agents, ResourcePermission::NONE);

        let perms = resolve_permissions(OrgRole::Member, &overrides);

        // Member default is r-x — override can't remove, only add
        assert!(perms.can_read(ResourceType::Agents));
        assert!(perms.can_execute(ResourceType::Agents));
    }
}
