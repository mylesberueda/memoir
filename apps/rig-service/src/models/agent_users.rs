pub(crate) use super::_entity::agent_users::*;
use sea_orm::{ActiveModelBehavior, ColumnTrait as _, ConnectionTrait, EntityTrait as _, QueryFilter as _};

/// Cursor for agent share pagination: (created_at, id)
pub(crate) type ShareCursor = super::cursor::Cursor<(chrono::NaiveDateTime, i64)>;

impl ActiveModelBehavior for ActiveModel {}

/// Bitfield permission type for resource sharing.
///
/// Stored as `i16` in the DB. Use named constants and methods —
/// never raw bit literals outside this definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Permissions(i16);

impl Permissions {
    pub const NONE: Self = Self(0);
    pub const READ: Self = Self(1);
    pub const WRITE: Self = Self(2);
    pub const EXECUTE: Self = Self(4);
    pub const RWX: Self = Self(7);

    pub const fn can_read(self) -> bool {
        self.0 & Self::READ.0 != 0
    }

    pub const fn can_write(self) -> bool {
        self.0 & Self::WRITE.0 != 0
    }

    pub const fn can_execute(self) -> bool {
        self.0 & Self::EXECUTE.0 != 0
    }

    pub const fn value(self) -> i16 {
        self.0
    }
}

impl std::ops::BitOr for Permissions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for Permissions {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl From<i16> for Permissions {
    fn from(value: i16) -> Self {
        Self(value)
    }
}

impl From<Permissions> for i16 {
    fn from(perms: Permissions) -> Self {
        perms.0
    }
}

impl From<Permissions> for sea_orm::Value {
    fn from(perms: Permissions) -> Self {
        sea_orm::Value::SmallInt(Some(perms.0))
    }
}

impl std::fmt::Display for Permissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = if self.can_read() { 'r' } else { '-' };
        let w = if self.can_write() { 'w' } else { '-' };
        let x = if self.can_execute() { 'x' } else { '-' };
        write!(f, "{r}{w}{x}")
    }
}

impl Model {
    pub(crate) fn permissions(&self) -> Permissions {
        Permissions::from(self.permissions)
    }
}

impl Entity {
    /// Check if a user has at least the required permission on an agent.
    pub(crate) async fn has_permission<C>(
        db: &C,
        agent_id: i64,
        user_id: &str,
        required: Permissions,
    ) -> Result<bool, sea_orm::DbErr>
    where
        C: ConnectionTrait,
    {
        let share = Self::find()
            .filter(Column::AgentId.eq(agent_id))
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?;

        Ok(share.is_some_and(|s| (Permissions::from(s.permissions) & required) == required))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_represent_no_permissions() {
        let p = Permissions::NONE;
        assert!(!p.can_read());
        assert!(!p.can_write());
        assert!(!p.can_execute());
        assert_eq!(p.to_string(), "---");
    }

    #[test]
    fn should_represent_read_only() {
        let p = Permissions::READ;
        assert!(p.can_read());
        assert!(!p.can_write());
        assert!(!p.can_execute());
        assert_eq!(p.to_string(), "r--");
    }

    #[test]
    fn should_combine_permissions_with_bitor() {
        let p = Permissions::READ | Permissions::EXECUTE;
        assert!(p.can_read());
        assert!(!p.can_write());
        assert!(p.can_execute());
        assert_eq!(p.to_string(), "r-x");
    }

    #[test]
    fn should_represent_full_permissions() {
        let p = Permissions::RWX;
        assert!(p.can_read());
        assert!(p.can_write());
        assert!(p.can_execute());
        assert_eq!(p.to_string(), "rwx");
    }

    #[test]
    fn should_roundtrip_through_i16() {
        let p = Permissions::READ | Permissions::WRITE;
        let raw: i16 = p.into();
        let restored = Permissions::from(raw);
        assert_eq!(p, restored);
    }

    #[test]
    fn should_check_required_permissions_with_bitand() {
        let granted = Permissions::READ | Permissions::EXECUTE;
        let required = Permissions::EXECUTE;
        assert_eq!(granted & required, required);

        let missing = Permissions::WRITE;
        assert_ne!(granted & missing, missing);
    }
}
