pub(crate) use super::_entity::conversations::*;
use crate::models::conversation_users;
use crate::{
    api::{conversation::Conversation, message::Message},
    models::{agents, conversation_documents, documents, messages},
};
use platform_rs::cache::{ResolvedPermissions, ResourceType};
use proto_rs::rig::v1;
use sea_orm::sea_query::{BinOper, Expr, ExprTrait as _};
use sea_orm::{
    ActiveModelBehavior, ColumnTrait as _, Condition, ConnectionTrait, EntityTrait as _, JoinType, QueryFilter as _,
    QueryOrder as _, QuerySelect as _, RelationTrait as _, Select,
};

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ConversationError {
    #[error(transparent)]
    Query(#[from] sea_orm::DbErr),
    #[error("conversation not found")]
    NotFound,
    #[error("conversation missing agent")]
    MissingAgent,
}

impl Entity {
    /// Returns a `Select` with authorization filters applied.
    ///
    /// Conversations are user-scoped within an org, so the org branch checks
    /// both org membership AND user ownership. Shares bypass both gates.
    pub(crate) fn authorized_query(
        pid: &str,
        user_id: &str,
        org_pid: &str,
        org_perms: &ResolvedPermissions,
        required: conversation_users::Permissions,
    ) -> Select<Self> {
        let type_allows = match required {
            p if p == conversation_users::Permissions::READ => org_perms.can_read(ResourceType::Conversations),
            p if p == conversation_users::Permissions::WRITE => org_perms.can_write(ResourceType::Conversations),
            p if p == conversation_users::Permissions::EXECUTE => org_perms.can_execute(ResourceType::Conversations),
            _ => false,
        };

        let mask = required.value();
        let share_filter = Condition::all()
            .add(conversation_users::Column::UserId.eq(user_id))
            .add(
                Expr::col(conversation_users::Column::Permissions)
                    .binary(BinOper::BitAnd, mask)
                    .ne(0),
            );

        let mut query = Self::find()
            .filter(Column::Pid.eq(pid))
            .filter(Column::IsDeleted.eq(false));

        if type_allows {
            query = query
                .join(JoinType::LeftJoin, Relation::ConversationUsers.def())
                .filter(
                    Condition::any()
                        .add(
                            Condition::all()
                                .add(Column::OrganizationPid.eq(org_pid))
                                .add(Column::UserId.eq(user_id)),
                        )
                        .add(share_filter),
                );
        } else {
            query = query
                .join(JoinType::InnerJoin, Relation::ConversationUsers.def())
                .filter(share_filter);
        }

        query
    }

    pub(crate) async fn find_conversation_by_pid<C>(
        db: &C,
        pid: &str,
        user_id: &str,
    ) -> Result<Conversation, ConversationError>
    where
        C: ConnectionTrait,
    {
        let (conversation, agent) = Self::find()
            .filter(Column::Pid.eq(pid))
            .filter(Column::IsDeleted.eq(false))
            .join(JoinType::LeftJoin, Relation::ConversationUsers.def())
            .filter(
                Condition::any()
                    .add(Column::UserId.eq(user_id))
                    .add(conversation_users::Column::UserId.eq(user_id)),
            )
            .find_also_related(agents::Entity)
            .one(db)
            .await?
            .ok_or(ConversationError::NotFound)?;

        let agent = agent.ok_or(ConversationError::MissingAgent)?;

        let messages: Vec<v1::Message> = messages::Entity::find()
            .filter(messages::Column::ConversationId.eq(conversation.id))
            .filter(messages::Column::IsDeleted.eq(false))
            .order_by_asc(messages::Column::CreatedAt)
            .order_by_asc(messages::Column::Id)
            .all(db)
            .await?
            .into_iter()
            .map(|model| {
                let msg: Message = model.into();
                msg.into()
            })
            .collect();

        Ok(Self::assemble_conversation(conversation, agent.pid, messages))
    }

    pub(crate) fn assemble_conversation(model: Model, agent_pid: String, messages: Vec<v1::Message>) -> Conversation {
        Conversation {
            model,
            agent_pid,
            messages,
        }
    }

    /// Query all conversation PIDs that a document is attached to.
    /// The `conversation_documents` join table is the source of truth; this result
    /// is used to update the denormalized `conversation_pids` array in Qdrant.
    pub(crate) async fn get_conversation_pids_for_document(
        db: &sea_orm::DatabaseConnection,
        doc_pid: &str,
    ) -> Result<Vec<String>, sea_orm::DbErr> {
        Entity::find()
            .select_only()
            .column(Column::Pid)
            .join(JoinType::InnerJoin, Relation::ConversationDocuments.def())
            .join(JoinType::InnerJoin, conversation_documents::Relation::Documents.def())
            .filter(documents::Column::Pid.eq(doc_pid))
            .filter(Column::IsDeleted.eq(false))
            .into_tuple::<String>()
            .all(db)
            .await
    }
}
