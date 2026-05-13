//! Session registry lifecycle hook.

use super::{Hooks, HooksError};
use crate::{AppContext, actors::SessionRegistryActor, api::store::PostgresStore, tools::ToolRegistry};
use kameo::{actor::ActorRef, actor::Spawn as _};
use std::sync::Arc;

pub(crate) struct Registry {
    ctx: Arc<AppContext>,
    tool_registry: ToolRegistry,
}

impl Registry {
    pub(crate) fn new(ctx: Arc<AppContext>, tool_registry: ToolRegistry) -> Self {
        Self { ctx, tool_registry }
    }

    pub(crate) async fn init(
        ctx: Arc<AppContext>,
        tool_registry: ToolRegistry,
    ) -> Result<ActorRef<SessionRegistryActor<PostgresStore>>, HooksError> {
        Self::new(ctx, tool_registry).on_startup().await
    }
}

impl Hooks<ActorRef<SessionRegistryActor<PostgresStore>>> for Registry {
    async fn on_startup(&self) -> Result<ActorRef<SessionRegistryActor<PostgresStore>>, HooksError> {
        let store = Arc::new(PostgresStore::new(self.ctx.db.clone()));
        let registry = SessionRegistryActor::spawn((store, self.ctx.clone(), self.tool_registry.clone()));

        tracing::info!("Session registry initialized!");
        Ok(registry)
    }
}
