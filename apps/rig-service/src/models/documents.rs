pub(crate) use super::_entity::documents::*;
use proto_rs::rig::v1::{Document, DocumentStatus};
use sea_orm::{ActiveModelBehavior, ColumnTrait as _, DatabaseConnection, EntityTrait as _, QueryFilter as _};

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumString, strum::Display)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum DocStatus {
    Pending,
    Processing,
    Ready,
    Failed,
}

impl From<DocStatus> for String {
    fn from(status: DocStatus) -> Self {
        status.to_string()
    }
}

impl From<DocStatus> for DocumentStatus {
    fn from(status: DocStatus) -> Self {
        match status {
            DocStatus::Pending => DocumentStatus::Pending,
            DocStatus::Processing => DocumentStatus::Processing,
            DocStatus::Ready => DocumentStatus::Ready,
            DocStatus::Failed => DocumentStatus::Failed,
        }
    }
}

impl Entity {
    /// Update document status and optional error message.
    pub(crate) async fn set_status(
        db: &DatabaseConnection,
        doc_id: i64,
        status: DocStatus,
        error_message: Option<String>,
    ) {
        let result = Self::update_many()
            .col_expr(Column::Status, sea_orm::sea_query::Expr::value(status.to_string()))
            .col_expr(Column::ErrorMessage, sea_orm::sea_query::Expr::value(error_message))
            .filter(Column::Id.eq(doc_id))
            .exec(db)
            .await;

        match result {
            Ok(_) => tracing::debug!(document_id = doc_id, %status, "document status updated"),
            Err(e) => tracing::error!(document_id = doc_id, error = %e, "failed to update document status"),
        }
    }
}

impl Model {
    pub(crate) fn status(&self) -> DocStatus {
        self.status.parse().unwrap_or_else(|_| {
            tracing::warn!(document_id = self.id, status = %self.status, "unknown document status, defaulting to Pending");
            DocStatus::Pending
        })
    }

    /// Access: owner always has access, or org member (same org) has access.
    pub(crate) fn is_accessible(&self, user_id: &str, organization_pid: &str) -> bool {
        self.user_id == user_id || self.organization_pid == organization_pid
    }

    pub(crate) fn into_proto(self) -> Document {
        let status: DocumentStatus = self.status().into();
        Document {
            pid: self.pid,
            filename: self.filename,
            content_type: self.content_type,
            size_bytes: self.size_bytes,
            summary: self.summary,
            status: status.into(),
            error_message: self.error_message,
            created_at: self.created_at.and_utc().to_rfc3339(),
            updated_at: self.updated_at.and_utc().to_rfc3339(),
        }
    }
}

impl ModelEx {
    pub(crate) fn into_proto(self) -> Document {
        Model::from(self).into_proto()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::prelude::DateTime;

    fn make_model(status: &str) -> Model {
        Model {
            id: 1,
            pid: "doc_test".to_string(),
            user_id: "user_1".to_string(),
            organization_pid: "org_1".to_string(),
            filename: "test.txt".to_string(),
            content_type: "text/plain".to_string(),
            size_bytes: 1024,
            storage_path: "org_1/doc_test/test.txt".to_string(),
            summary: None,
            status: status.to_string(),
            error_message: None,
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
    }

    mod status {
        use super::*;

        #[test]
        fn should_parse_pending_status() {
            let model = make_model("pending");
            assert_eq!(model.status(), DocStatus::Pending);
        }

        #[test]
        fn should_parse_processing_status() {
            let model = make_model("processing");
            assert_eq!(model.status(), DocStatus::Processing);
        }

        #[test]
        fn should_parse_ready_status() {
            let model = make_model("ready");
            assert_eq!(model.status(), DocStatus::Ready);
        }

        #[test]
        fn should_parse_failed_status() {
            let model = make_model("failed");
            assert_eq!(model.status(), DocStatus::Failed);
        }

        #[test]
        fn should_default_to_pending_for_unknown_status() {
            let model = make_model("bogus");
            assert_eq!(model.status(), DocStatus::Pending);
        }
    }

    mod access_control {
        use super::*;

        #[test]
        fn should_allow_access_for_owner() {
            let model = make_model("ready");
            assert!(model.is_accessible("user_1", "org_1"));
        }

        #[test]
        fn should_allow_access_for_owner_regardless_of_org() {
            let model = make_model("ready");
            assert!(model.is_accessible("user_1", "different_org"));
        }

        #[test]
        fn should_allow_access_for_org_member() {
            let model = make_model("ready");
            assert!(model.is_accessible("user_2", "org_1"));
        }

        #[test]
        fn should_deny_access_for_wrong_user_and_wrong_org() {
            let model = make_model("ready");
            assert!(!model.is_accessible("user_2", "org_2"));
        }
    }
}
