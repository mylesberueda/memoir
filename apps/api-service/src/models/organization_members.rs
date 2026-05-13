pub(crate) use super::_entity::organization_members::*;
use proto_rs::api::v1::OrganizationMember;
use sea_orm::ActiveModelBehavior;

// OrgRole is defined in common-rs. Re-export as OrganizationRole for api-service usage.
pub(crate) use platform_rs::cache::OrgRole as OrganizationRole;

/// Cursor for member pagination: (role, created_at, id)
pub(crate) type MemberCursor = super::cursor::Cursor<(String, chrono::NaiveDateTime, i32)>;

/// App-level conversion from String with tonic::Status error.
/// Use this for proto request parsing where an invalid role is a client error.
pub(crate) fn parse_role(role: String) -> Result<OrganizationRole, tonic::Status> {
    role.parse()
        .map_err(|_| tonic::Status::invalid_argument("Invalid organization role."))
}

impl Model {
    #[allow(dead_code)]
    pub(crate) fn role(&self) -> Result<OrganizationRole, tonic::Status> {
        self.role.parse().map_err(|_| {
            tracing::error!(member_id = %self.id, role = %self.role, "invalid organization role in database");
            tonic::Status::internal("Invalid role data")
        })
    }
}

impl From<Model> for OrganizationMember {
    fn from(member: Model) -> Self {
        Self {
            pid: member.pid,
            organization_id: member.organization_id.to_string(),
            user_id: member.user_id,
            role: member.role.clone(),
            created_at: member.created_at.and_utc().to_rfc3339(),
            display_name: None,
            email: String::new(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
