#![allow(clippy::enum_variant_names)] // join tables can have the same suffixes
#![allow(unused)] // TODO(_): Remove after impl

mod _entity;
pub(crate) mod agent_secrets;
pub(crate) mod agent_tools;
pub(crate) mod agent_users;
pub(crate) mod agents;
pub(crate) mod conversation_documents;
pub(crate) mod conversation_users;
pub(crate) mod conversations;
pub(crate) mod cursor;
pub(crate) mod document_group_memberships;
pub(crate) mod document_groups;
pub(crate) mod documents;
pub(crate) mod language_models;
pub(crate) mod messages;
pub(crate) mod provider_rates;
pub(crate) mod provider_secrets;
pub(crate) mod providers;
pub(crate) mod secrets;
pub(crate) mod tool_secrets;
pub(crate) mod tools;
pub(crate) mod user_assistants;
