pub(crate) use super::_entity::conversation_users::*;
pub(crate) use super::agent_users::{Permissions, ShareCursor};
use sea_orm::{ActiveModelBehavior, ColumnTrait as _, ConnectionTrait, EntityTrait as _, QueryFilter as _};

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub(crate) fn permissions(&self) -> Permissions {
        Permissions::from(self.permissions)
    }
}

impl Entity {
    /// Check if a user has at least the required permission on a conversation.
    pub(crate) async fn has_permission<C>(
        db: &C,
        conversation_id: i64,
        user_id: &str,
        required: Permissions,
    ) -> Result<bool, sea_orm::DbErr>
    where
        C: ConnectionTrait,
    {
        let share = Self::find()
            .filter(Column::ConversationId.eq(conversation_id))
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?;

        Ok(share.is_some_and(|s| (Permissions::from(s.permissions) & required) == required))
    }
}
