//! Tool registry lifecycle hook.

use super::{Hooks, HooksError};
use crate::{AppContext, tools::ToolRegistry};
use std::sync::Arc;

pub(crate) struct Registry {
    ctx: Arc<AppContext>,
}

impl Registry {
    pub(crate) fn new(ctx: Arc<AppContext>) -> Self {
        Self { ctx }
    }

    pub(crate) async fn init(ctx: Arc<AppContext>) -> Result<ToolRegistry, HooksError> {
        Self::new(ctx).on_startup().await
    }
}

impl Hooks<ToolRegistry> for Registry {
    async fn on_startup(&self) -> Result<ToolRegistry, HooksError> {
        let registry = ToolRegistry::new(self.ctx.clone());

        tracing::info!("Tool registry initialized!");
        Ok(registry)
    }
}
