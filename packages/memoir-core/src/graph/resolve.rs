//! Mapping extracted entity strings to canonical graph nodes.
//!
//! A [`Triple`](crate::graph::Triple)'s `subject` and `object` are bare strings
//! the extractor produced; two memories that mention the same real-world entity
//! under different surface forms ("Alice", "Alice Smith") must resolve to the
//! same graph node or the graph fragments into duplicates. [`EntityResolver`] is
//! the seam that maps a candidate string to either an existing node or a new
//! one.
//!
//! Two implementations ship: [`ExactStringResolver`] (the benchmark floor —
//! match on canonical name only) and [`EmbeddingEntityResolver`] (the
//! production impl — cosine similarity over candidate embeddings). Both partition
//! the resolution space by [`Scope`] so an entity in one tenant never merges with
//! a like-named entity in another.
//!
//! The candidates an embedding resolver compares against come from an
//! [`EntityCatalog`], a focused retrieval seam separate from
//! [`GraphStore`](crate::graph::GraphStore): the resolver needs only
//! "the entities I might merge with in this scope", not the full store surface.
//! [`InMemoryEntityCatalog`] is the always-available backing for tests and
//! benchmarks; a FalkorDB-backed catalog (reading the nodes the commit path
//! writes) is layered on in the commit ticket.

use std::collections::HashMap;
use std::future::Future;
use std::sync::Mutex;

use crate::embedding::{EmbeddingError, EmbeddingModel};
use crate::memory::Scope;

/// Minimum cosine similarity for two entity strings to be the same node.
///
/// Below this floor the surface forms are treated as distinct entities and a
/// new node is created. Mirrors the `MIN_CATEGORY_SCORE` precedent
/// (`client/categorize.rs`): a deliberately conservative default that favors a
/// new node over a wrong merge, since an over-eager merge is harder to undo than
/// a duplicate.
pub const MIN_ENTITY_SIMILARITY: f32 = 0.85;

/// The outcome of resolving an entity string against the existing graph.
///
/// Tells the commit path whether to match an existing node or create one, and
/// carries the canonical name so surface-form variants collapse onto a single
/// node. A resolver that finds a match returns [`Resolution::Existing`] with the
/// matched node's stable key; otherwise [`Resolution::New`] with the name to
/// create.
#[derive(Debug, Clone, PartialEq)]
pub enum Resolution {
    /// The entity matched an existing node identified by `key`.
    Existing {
        /// The matched node's stable identity within its scope.
        key: String,
        /// The matched node's canonical name (may differ from the query string).
        name: String,
    },

    /// No existing node matched; a node should be created under `name`.
    New {
        /// The canonical name to create the node under.
        name: String,
    },
}

/// A candidate entity node the resolver compares a query against.
///
/// `key` is the node's stable identity within its scope; `name` is its canonical
/// surface form; `embedding` is the vector of `name`, used for cosine matching.
/// Yielded by an [`EntityCatalog`] for one scope.
#[derive(Debug, Clone, PartialEq)]
pub struct EntityVector {
    /// The node's stable identity within its scope.
    pub key: String,
    /// The node's canonical name.
    pub name: String,
    /// The embedding of `name`, for cosine comparison.
    pub embedding: Vec<f32>,
}

/// Yields the existing entity nodes a resolver may merge a candidate into.
///
/// A focused retrieval seam, deliberately narrower than
/// [`GraphStore`](crate::graph::GraphStore): the embedding resolver needs only
/// the candidate entities in one [`Scope`], not the full store surface. This is
/// also the seam where a FalkorDB-native vector index later replaces the linear
/// scan, behind the same trait, without changing the resolver.
pub trait EntityCatalog: Send + Sync + 'static {
    /// Returns the entity nodes within `scope` that a candidate may match.
    ///
    /// Scope-confined: implementations never yield entities from another scope,
    /// upholding the same tenant isolation the rest of memoir enforces.
    ///
    /// # Errors
    ///
    /// Returns [`ResolveError::Catalog`] when the backing store cannot be read.
    fn candidates_in_scope(
        &self,
        scope: &Scope,
    ) -> impl Future<Output = Result<Vec<EntityVector>, ResolveError>> + Send;
}

/// Maps an extracted entity string to a canonical graph node.
///
/// Implementations decide whether a candidate is an existing node or a new one,
/// always confined to the candidate's [`Scope`]. Swapping one implementation for
/// another (exact-string, embedding, or a future ANN-backed impl) requires no
/// caller change, which is what lets the benchmark compare them.
pub trait EntityResolver: Send + Sync + 'static {
    /// Resolves `entity` within `scope` to an existing or new node.
    ///
    /// # Errors
    ///
    /// Returns [`ResolveError::Catalog`] when candidate retrieval fails and
    /// [`ResolveError::Embed`] when the embedding model fails.
    fn resolve(&self, scope: &Scope, entity: &str) -> impl Future<Output = Result<Resolution, ResolveError>> + Send;
}

/// Failure modes for [`EntityResolver`] implementations.
#[derive(Debug, thiserror::Error)]
pub enum ResolveError {
    /// Reading candidate entities from the [`EntityCatalog`] failed.
    #[error("entity catalog read failed: {0}")]
    Catalog(String),

    /// Embedding the candidate entity string failed.
    #[error("entity embedding failed: {0}")]
    Embed(#[from] EmbeddingError),
}

/// Resolves entities by exact canonical-name match within a scope.
///
/// The benchmark floor: two surface forms of the same entity ("Alice", "Alice
/// Smith") resolve to *different* nodes because only an exact name match counts.
/// Useful as the baseline [`EmbeddingEntityResolver`] is measured against, and as
/// a zero-dependency resolver when embedding-based merging is not wanted.
pub struct ExactStringResolver<C> {
    catalog: C,
}

impl<C: EntityCatalog> ExactStringResolver<C> {
    /// Builds an exact-match resolver over `catalog`.
    pub fn new(catalog: C) -> Self {
        Self { catalog }
    }
}

impl<C: EntityCatalog> EntityResolver for ExactStringResolver<C> {
    async fn resolve(&self, scope: &Scope, entity: &str) -> Result<Resolution, ResolveError> {
        let candidates = self.catalog.candidates_in_scope(scope).await?;
        match candidates.into_iter().find(|candidate| candidate.name == entity) {
            Some(matched) => Ok(Resolution::Existing {
                key: matched.key,
                name: matched.name,
            }),
            None => Ok(Resolution::New {
                name: entity.to_string(),
            }),
        }
    }
}

/// Resolves entities by cosine similarity over candidate embeddings.
///
/// The production impl: embeds the candidate string and compares it against the
/// embeddings of existing nodes in the same scope, merging into the closest node
/// above [`MIN_ENTITY_SIMILARITY`]. This collapses surface-form variants onto one
/// node where the exact-string resolver would fragment them.
///
/// Candidate embeddings are stored as a node property and scanned exactly here;
/// a FalkorDB-native vector index can later replace the catalog's linear scan
/// behind the [`EntityCatalog`] seam without changing this resolver.
///
/// Generic over the embedder and catalog so tests inject stubs.
pub struct EmbeddingEntityResolver<E, C> {
    embedder: E,
    catalog: C,
    min_similarity: f32,
}

impl<E: EmbeddingModel, C: EntityCatalog> EmbeddingEntityResolver<E, C> {
    /// Builds a resolver over `embedder` and `catalog` with the default floor.
    pub fn new(embedder: E, catalog: C) -> Self {
        Self {
            embedder,
            catalog,
            min_similarity: MIN_ENTITY_SIMILARITY,
        }
    }

    /// Overrides the minimum cosine similarity for a match.
    #[must_use]
    pub fn with_min_similarity(mut self, min_similarity: f32) -> Self {
        self.min_similarity = min_similarity;
        self
    }
}

impl<E: EmbeddingModel, C: EntityCatalog> EntityResolver for EmbeddingEntityResolver<E, C> {
    async fn resolve(&self, scope: &Scope, entity: &str) -> Result<Resolution, ResolveError> {
        let query = self.embedder.embed(entity).await?;
        let candidates = self.catalog.candidates_in_scope(scope).await?;

        let best = candidates
            .into_iter()
            .filter_map(|candidate| cosine_similarity(&query, &candidate.embedding).map(|score| (score, candidate)))
            .filter(|(score, _)| *score >= self.min_similarity)
            .max_by(|(a, _), (b, _)| a.total_cmp(b));

        match best {
            Some((_, matched)) => Ok(Resolution::Existing {
                key: matched.key,
                name: matched.name,
            }),
            None => Ok(Resolution::New {
                name: entity.to_string(),
            }),
        }
    }
}

/// Returns the cosine similarity of `a` and `b`, or `None` if undefined.
///
/// Undefined when the vectors differ in length or either has zero magnitude;
/// callers treat `None` as "not a match" rather than a hard error so one
/// malformed candidate does not fail the whole resolution.
fn cosine_similarity(a: &[f32], b: &[f32]) -> Option<f32> {
    if a.len() != b.len() || a.is_empty() {
        return None;
    }

    let mut dot = 0.0f32;
    let mut norm_a = 0.0f32;
    let mut norm_b = 0.0f32;
    for (x, y) in a.iter().zip(b) {
        dot += x * y;
        norm_a += x * x;
        norm_b += y * y;
    }

    let denom = norm_a.sqrt() * norm_b.sqrt();
    if denom == 0.0 { None } else { Some(dot / denom) }
}

/// In-memory [`EntityCatalog`] for tests and benchmarks, with no live backend.
///
/// Holds entity vectors per scope so a resolver's matching logic runs against
/// real candidates rather than a faked query result. This is the catalog's
/// test/benchmark boundary, mirroring
/// [`InMemoryGraphStore`](crate::graph::InMemoryGraphStore).
///
/// # Examples
///
/// ```
/// use memoir_core::graph::{EntityCatalog, EntityVector, InMemoryEntityCatalog};
/// use memoir_core::memory::Scope;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let scope = Scope { agent_id: "a".into(), org_id: "o".into(), user_id: "u".into() };
/// let catalog = InMemoryEntityCatalog::new();
/// catalog.insert(&scope, EntityVector { key: "e1".into(), name: "Alice".into(), embedding: vec![1.0, 0.0] });
///
/// let found = catalog.candidates_in_scope(&scope).await?;
/// assert_eq!(found.len(), 1);
/// # Ok(())
/// # }
/// ```
#[derive(Default)]
pub struct InMemoryEntityCatalog {
    by_scope: Mutex<HashMap<Scope, Vec<EntityVector>>>,
}

impl InMemoryEntityCatalog {
    /// Creates an empty catalog.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds `entity` to `scope`'s candidate set.
    pub fn insert(&self, scope: &Scope, entity: EntityVector) {
        self.by_scope
            .lock()
            .expect("entity catalog mutex poisoned")
            .entry(scope.clone())
            .or_default()
            .push(entity);
    }
}

impl EntityCatalog for InMemoryEntityCatalog {
    async fn candidates_in_scope(&self, scope: &Scope) -> Result<Vec<EntityVector>, ResolveError> {
        Ok(self
            .by_scope
            .lock()
            .expect("entity catalog mutex poisoned")
            .get(scope)
            .cloned()
            .unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scope(user: &str) -> Scope {
        Scope {
            agent_id: "agent".to_string(),
            org_id: "org".to_string(),
            user_id: user.to_string(),
        }
    }

    /// Embeds a name to a fixed unit vector so cosine is deterministic in tests.
    ///
    /// "Alice" and "Alice Smith" map to near-identical vectors (the surface-form
    /// pair the embedding resolver must merge and the exact resolver must not);
    /// "Bob" maps to an orthogonal vector.
    struct FakeEmbedding;

    impl EmbeddingModel for FakeEmbedding {
        async fn embed(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
            let vector = if text.starts_with("Alice") {
                vec![1.0, 0.0, 0.0]
            } else if text == "Bob" {
                vec![0.0, 1.0, 0.0]
            } else {
                vec![0.0, 0.0, 1.0]
            };
            Ok(vector)
        }

        fn dimensions(&self) -> usize {
            3
        }
    }

    async fn catalog_with_alice() -> InMemoryEntityCatalog {
        let catalog = InMemoryEntityCatalog::new();
        catalog.insert(
            &scope("u"),
            EntityVector {
                key: "alice-node".to_string(),
                name: "Alice".to_string(),
                embedding: vec![1.0, 0.0, 0.0],
            },
        );
        catalog
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_match_exact_name_with_exact_resolver() {
        let resolver = ExactStringResolver::new(catalog_with_alice().await);

        let resolution = resolver.resolve(&scope("u"), "Alice").await.unwrap();

        assert_eq!(
            resolution,
            Resolution::Existing {
                key: "alice-node".to_string(),
                name: "Alice".to_string(),
            }
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_create_new_for_surface_variant_with_exact_resolver() {
        let resolver = ExactStringResolver::new(catalog_with_alice().await);

        let resolution = resolver.resolve(&scope("u"), "Alice Smith").await.unwrap();

        assert_eq!(
            resolution,
            Resolution::New {
                name: "Alice Smith".to_string(),
            }
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_merge_surface_variant_with_embedding_resolver() {
        let resolver = EmbeddingEntityResolver::new(FakeEmbedding, catalog_with_alice().await);

        let resolution = resolver.resolve(&scope("u"), "Alice Smith").await.unwrap();

        assert_eq!(
            resolution,
            Resolution::Existing {
                key: "alice-node".to_string(),
                name: "Alice".to_string(),
            }
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_create_new_for_dissimilar_entity_with_embedding_resolver() {
        let resolver = EmbeddingEntityResolver::new(FakeEmbedding, catalog_with_alice().await);

        let resolution = resolver.resolve(&scope("u"), "Bob").await.unwrap();

        assert_eq!(
            resolution,
            Resolution::New {
                name: "Bob".to_string()
            }
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn should_not_merge_across_scopes() {
        let resolver = EmbeddingEntityResolver::new(FakeEmbedding, catalog_with_alice().await);

        let resolution = resolver.resolve(&scope("other"), "Alice").await.unwrap();

        assert_eq!(
            resolution,
            Resolution::New {
                name: "Alice".to_string()
            }
        );
    }

    #[test]
    fn should_compute_cosine_for_identical_vectors() {
        let similarity = cosine_similarity(&[1.0, 0.0], &[1.0, 0.0]).unwrap();
        assert!((similarity - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn should_return_none_cosine_for_mismatched_lengths() {
        assert_eq!(cosine_similarity(&[1.0, 0.0], &[1.0]), None);
    }

    #[test]
    fn should_return_none_cosine_for_zero_magnitude() {
        assert_eq!(cosine_similarity(&[0.0, 0.0], &[1.0, 0.0]), None);
    }
}
