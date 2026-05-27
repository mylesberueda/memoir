// SeaORM entities generated against the auth schema (migrations 000003-000005).
// Regenerate with `nx run memoir-service:gen:entities` after schema changes.
//
// Security note: generated Model structs expose hash columns (`password_hash`,
// `key_hash`, `token_hash`) as plain `String` with `Serialize` derived. Handlers
// MUST map entities to proto types at the handler boundary rather than
// serializing entities directly — proto types don't include hash fields, so
// the wire surface stays safe by construction.
pub mod _entity;

pub use _entity::prelude::*;
