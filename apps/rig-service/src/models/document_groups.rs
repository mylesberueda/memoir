pub(crate) use super::_entity::document_groups::*;
use proto_rs::rig::v1::DocumentGroup;
use sea_orm::ActiveModelBehavior;

impl Model {
    pub(crate) fn is_owner(&self, user_id: &str) -> bool {
        self.user_id == user_id
    }

    /// Access: owner always has access. Org members have access if is_org_shared
    /// and the group belongs to the same org.
    pub(crate) fn is_accessible(&self, user_id: &str, organization_pid: &str) -> bool {
        self.is_owner(user_id) || (self.is_org_shared && self.organization_pid == organization_pid)
    }

    pub(crate) fn into_proto(self, document_count: i32) -> DocumentGroup {
        DocumentGroup {
            pid: self.pid,
            name: self.name,
            description: self.description,
            is_org_shared: self.is_org_shared,
            document_count,
            created_at: self.created_at.and_utc().to_rfc3339(),
            updated_at: self.updated_at.and_utc().to_rfc3339(),
        }
    }
}

impl ModelEx {
    pub(crate) fn into_proto(self, document_count: i32) -> DocumentGroup {
        Model::from(self).into_proto(document_count)
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::prelude::DateTime;

    fn make_model(is_org_shared: bool) -> Model {
        Model {
            id: 1,
            pid: "grp_test".to_string(),
            user_id: "user_1".to_string(),
            organization_pid: "org_1".to_string(),
            name: "Test Group".to_string(),
            description: None,
            is_org_shared,
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
    }

    mod ownership {
        use super::*;

        #[test]
        fn should_confirm_owner() {
            let model = make_model(false);
            assert!(model.is_owner("user_1"));
        }

        #[test]
        fn should_deny_non_owner() {
            let model = make_model(false);
            assert!(!model.is_owner("user_2"));
        }
    }

    mod access_control {
        use super::*;

        #[test]
        fn should_allow_access_for_owner() {
            let model = make_model(false);
            assert!(model.is_accessible("user_1", "org_1"));
        }

        #[test]
        fn should_allow_access_for_owner_regardless_of_org() {
            let model = make_model(false);
            assert!(model.is_accessible("user_1", "different_org"));
        }

        #[test]
        fn should_allow_access_for_org_member_when_shared() {
            let model = make_model(true);
            assert!(model.is_accessible("user_2", "org_1"));
        }

        #[test]
        fn should_deny_access_for_org_member_when_not_shared() {
            let model = make_model(false);
            assert!(!model.is_accessible("user_2", "org_1"));
        }

        #[test]
        fn should_deny_access_for_wrong_org_even_when_shared() {
            let model = make_model(true);
            assert!(!model.is_accessible("user_2", "org_2"));
        }

        #[test]
        fn should_deny_access_for_wrong_user_and_wrong_org() {
            let model = make_model(true);
            assert!(!model.is_accessible("user_2", "org_2"));
        }
    }
}
