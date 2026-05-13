pub(crate) mod documents;
pub(crate) mod episodic_memory;
pub(crate) mod history;
pub(crate) mod registry;

pub(crate) use documents::DocumentsComponent;
pub(crate) use episodic_memory::EpisodicMemoryComponent;
pub(crate) use history::HistoryComponent;
#[allow(unused_imports, reason = "Used once wired into InferenceActor::load")]
pub(crate) use registry::ComponentRegistry;

use crate::{agents::rig::DynamicContextStore, api::message::Message};
use std::{future::Future, pin::Pin};

/// A composable unit of agent capability. Components hook into the agent's
/// inference lifecycle to provide functionality like memory, document context,
/// content moderation, etc.
///
/// Async methods return boxed futures for object safety — same pattern as
/// tonic/tower gRPC middleware.
///
/// Components are constructed with their dependencies, then `init()` is called
/// once during agent setup. Per-inference hooks fire on every `BaseAgent::infer()` call.
pub(crate) trait AgentComponent: Send + Sync {
    /// Component identity. Each impl declares `const NAME: &str` on the concrete
    /// type and returns `Self::NAME` here.
    #[allow(
        dead_code,
        reason = "Used for logging/debugging and in tests; will be called once components are ported"
    )]
    fn name(&self) -> &'static str;

    /// Called once during agent construction. Use this to register vector indices
    /// on the dynamic context store, or perform other one-time setup.
    fn init<'a>(
        &'a mut self,
        dynamic_context: &'a DynamicContextStore,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;

    /// System prompt section this component contributes. Returned sections are
    /// appended to the agent's system prompt during construction.
    fn system_prompt_section(&self) -> Option<&str> {
        None
    }

    /// Called before each inference. Can inspect or mutate the user message.
    fn on_pre_stream<'a>(&'a mut self, message: &'a mut Message) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        let _ = message;
        Box::pin(async {})
    }

    /// Called after each inference completes. Use for fire-and-forget work
    /// like embedding messages or extracting knowledge.
    fn on_post_stream<'a>(
        &'a self,
        user_message: &'a Message,
        assistant_message: &'a Message,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        let _ = (user_message, assistant_message);
        Box::pin(async {})
    }
}

#[cfg(test)]
mod agent_component_tests {
    use super::*;

    /// Minimal implementation for testing trait mechanics.
    struct StubComponent;

    impl StubComponent {
        const NAME: &str = "stub";
    }

    impl AgentComponent for StubComponent {
        fn name(&self) -> &'static str {
            Self::NAME
        }

        fn init<'a>(
            &'a mut self,
            _dynamic_context: &'a DynamicContextStore,
        ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
            Box::pin(async {})
        }
    }

    #[test]
    fn should_be_object_safe() {
        // If this compiles, the trait is dyn-compatible.
        let _: Box<dyn AgentComponent> = Box::new(StubComponent);
    }

    #[test]
    fn should_return_name_from_const() {
        let component = StubComponent;
        assert_eq!(component.name(), "stub");
    }

    #[test]
    fn should_return_none_for_default_system_prompt_section() {
        let component = StubComponent;
        assert!(component.system_prompt_section().is_none());
    }

    #[tokio::test]
    async fn should_not_panic_on_default_pre_stream() {
        let mut component = StubComponent;
        let mut message = crate::api::message::Message::new(
            crate::api::message::MessageRole::User,
            vec![crate::api::message::MessagePart::Text {
                id: "p1".into(),
                content: "hello".into(),
            }],
        );
        component.on_pre_stream(&mut message).await;
    }

    #[tokio::test]
    async fn should_not_panic_on_default_post_stream() {
        let component = StubComponent;
        let user_msg = crate::api::message::Message::new(
            crate::api::message::MessageRole::User,
            vec![crate::api::message::MessagePart::Text {
                id: "p1".into(),
                content: "hello".into(),
            }],
        );
        let assistant_msg = crate::api::message::Message::new(
            crate::api::message::MessageRole::Assistant,
            vec![crate::api::message::MessagePart::Text {
                id: "p2".into(),
                content: "hi there".into(),
            }],
        );
        component.on_post_stream(&user_msg, &assistant_msg).await;
    }

    #[test]
    fn should_store_heterogeneous_components_in_vec() {
        struct AnotherComponent;

        impl AnotherComponent {
            const NAME: &str = "another";
        }

        impl AgentComponent for AnotherComponent {
            fn name(&self) -> &'static str {
                Self::NAME
            }

            fn init<'a>(
                &'a mut self,
                _dynamic_context: &'a DynamicContextStore,
            ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
                Box::pin(async {})
            }

            fn system_prompt_section(&self) -> Option<&str> {
                Some("## Another\nThis is another component.")
            }
        }

        let components: Vec<Box<dyn AgentComponent>> = vec![Box::new(StubComponent), Box::new(AnotherComponent)];

        assert_eq!(components[0].name(), "stub");
        assert_eq!(components[1].name(), "another");
        assert!(components[0].system_prompt_section().is_none());
        assert!(components[1].system_prompt_section().is_some());
    }
}
