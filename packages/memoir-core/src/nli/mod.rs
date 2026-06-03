//! Zero-shot text classification via Natural Language Inference (NLI).
//!
//! Wraps a DeBERTa-v3-xsmall ONNX model (~87 MB quantized) trained on 33
//! classification datasets. Classification is framed as entailment: given the
//! input text as a premise and each candidate label rendered into a hypothesis
//! template, the model scores how strongly the premise entails the hypothesis.
//! The entailment probability ranks the labels.
//!
//! The model is downloaded from HuggingFace on first construction and cached
//! locally, mirroring how [`crate::embedding::OnnxEmbedding`] loads its model.
//!
//! # Execution providers
//!
//! By default the classifier runs on CPU. Enabling the `cuda` cargo feature
//! adds GPU execution: the classifier then negotiates CUDA-then-CPU and honors
//! the `NLI_EXECUTION_PROVIDER` environment variable (`auto` | `cuda` | `cpu`).
//! Enabling the feature does not change the public API — only which provider is
//! selected internally.
//!
//! # Examples
//!
//! ```no_run
//! use memoir_core::nli::{NliClassifier, NliConfig};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let classifier = NliClassifier::new(NliConfig::default())?;
//! let results = classifier.classify(
//!     "We decided to use Pulumi instead of Terraform",
//!     &["a decision that was made", "a personal preference"],
//!     "This text is about {}.",
//! )?;
//! // results[0].label == "a decision that was made"
//! # Ok(())
//! # }
//! ```

mod error;

#[doc(inline)]
pub use error::NliError;

use std::sync::Mutex;

use ort::session::Session;
use ort::value::Tensor;
use tokenizers::Tokenizer;

/// Default HuggingFace repo holding the pre-exported ONNX model.
///
/// MoritzLaurer's model is trained on 33 classification datasets, making it far
/// better at zero-shot classification than a plain NLI cross-encoder.
const DEFAULT_MODEL_REPO: &str = "MoritzLaurer/deberta-v3-xsmall-zeroshot-v1.1-all-33";

/// Default quantized ONNX model file within the repo (~87 MB).
const DEFAULT_MODEL_FILE: &str = "onnx/model_quantized.onnx";

/// Default tokenizer file within the repo.
const DEFAULT_TOKENIZER_FILE: &str = "tokenizer.json";

/// Source of the zero-shot NLI classifier model.
///
/// Selects which model [`NliClassifier::new`] downloads and loads for the
/// categorize stage. [`NliConfig::default`] is the model memoir ships with
/// (MoritzLaurer's DeBERTa-v3-xsmall); a consumer overrides it to point the
/// classifier at a different HuggingFace repo. Mirrors [`crate::llm::LlmConfig`]'s
/// enum-of-sources shape — one variant today, room to add others (a local path,
/// a direct URL) without a breaking change.
///
/// # Examples
///
/// ```
/// # use memoir_core::nli::NliConfig;
/// // The shipped default.
/// let config = NliConfig::default();
///
/// // A different zero-shot NLI repo, ONNX export + tokenizer.
/// let custom = NliConfig::huggingface(
///     "my-org/my-zeroshot-model",
///     "onnx/model.onnx",
///     "tokenizer.json",
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NliConfig {
    /// A model hosted on the HuggingFace Hub.
    HuggingFace {
        /// Repo id, e.g. `"MoritzLaurer/deberta-v3-xsmall-zeroshot-v1.1-all-33"`.
        repo: String,
        /// Path to the ONNX model file within the repo.
        model_file: String,
        /// Path to the tokenizer JSON within the repo.
        tokenizer_file: String,
    },
}

impl NliConfig {
    /// Builds a config for a model on the HuggingFace Hub.
    #[must_use]
    pub fn huggingface(
        repo: impl Into<String>,
        model_file: impl Into<String>,
        tokenizer_file: impl Into<String>,
    ) -> Self {
        Self::HuggingFace {
            repo: repo.into(),
            model_file: model_file.into(),
            tokenizer_file: tokenizer_file.into(),
        }
    }
}

impl Default for NliConfig {
    /// The model memoir ships with — MoritzLaurer's DeBERTa-v3-xsmall.
    fn default() -> Self {
        Self::huggingface(DEFAULT_MODEL_REPO, DEFAULT_MODEL_FILE, DEFAULT_TOKENIZER_FILE)
    }
}

/// Index of the entailment logit in the model's two-class output.
///
/// Output order is `[entailment, not_entailment]`, so entailment is index 0.
const ENTAILMENT_IDX: usize = 0;

/// A label paired with its entailment score from [`NliClassifier::classify`].
#[derive(Debug, Clone)]
pub struct ScoredLabel {
    /// The candidate label that was scored.
    pub label: String,

    /// Entailment probability in `[0.0, 1.0]`; higher means stronger match.
    pub score: f32,
}

/// The hardware backend an [`NliClassifier`] runs its inference on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionProvider {
    /// NVIDIA CUDA. Only reachable when the `cuda` cargo feature is enabled.
    Cuda,

    /// CPU (MLAS). The default, and the only option without the `cuda` feature.
    Cpu,
}

impl ExecutionProvider {
    /// Returns the ONNX Runtime identifier for this provider.
    #[must_use]
    pub fn ort_name(self) -> &'static str {
        match self {
            Self::Cuda => "CUDAExecutionProvider",
            Self::Cpu => "CPUExecutionProvider",
        }
    }
}

/// Zero-shot text classifier using NLI entailment scoring.
///
/// Holds a DeBERTa-v3-xsmall ONNX session and its tokenizer, each behind a
/// `Mutex` so the classifier is `Send + Sync` and can be shared via `Arc`. The
/// model is downloaded on first construction and cached locally.
pub struct NliClassifier {
    session: Mutex<Session>,
    tokenizer: Mutex<Tokenizer>,
    execution_provider: ExecutionProvider,
}

impl std::fmt::Debug for NliClassifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NliClassifier")
            .field("execution_provider", &self.execution_provider)
            .finish_non_exhaustive()
    }
}

impl NliClassifier {
    /// Creates a classifier from `config`, downloading the model if not cached.
    ///
    /// This is synchronous and blocks on the HuggingFace download and ONNX
    /// session creation — mirroring [`crate::embedding::OnnxEmbedding::new`].
    /// Call it from a blocking context (e.g. `tokio::task::spawn_blocking`)
    /// when constructing from async code. Pass [`NliConfig::default`] for the
    /// model memoir ships with.
    ///
    /// # Errors
    ///
    /// Returns [`NliError::Download`] if the model or tokenizer cannot be
    /// fetched, [`NliError::ModelLoad`] if the ONNX session cannot be built,
    /// and [`NliError::TokenizerLoad`] if the tokenizer cannot be parsed.
    pub fn new(config: NliConfig) -> Result<Self, NliError> {
        let NliConfig::HuggingFace {
            repo,
            model_file,
            tokenizer_file,
        } = config;

        let (model_path, tokenizer_path) = download_model_files(&repo, &model_file, &tokenizer_file)?;

        let (session, execution_provider) = create_session(&model_path)?;

        let tokenizer = Tokenizer::from_file(&tokenizer_path).map_err(|e| NliError::TokenizerLoad(e.to_string()))?;

        tracing::event!(
            name: "memoir.nli.loaded",
            tracing::Level::INFO,
            model = %repo,
            execution_provider = execution_provider.ort_name(),
            "NLI classifier loaded with {{execution_provider}}",
        );

        Ok(Self {
            session: Mutex::new(session),
            tokenizer: Mutex::new(tokenizer),
            execution_provider,
        })
    }

    /// Returns the execution provider this classifier resolved to at load time.
    #[must_use]
    pub fn execution_provider(&self) -> ExecutionProvider {
        self.execution_provider
    }

    /// Classifies `text` against `labels` using a hypothesis template.
    ///
    /// The template must contain `{}`, which is replaced with each label: with
    /// template `"This text is about {}."` and label `"a decision"`, the
    /// hypothesis is `"This text is about a decision."`. Returns the labels
    /// sorted by entailment score, highest first. An empty `labels` slice
    /// returns an empty vector.
    ///
    /// # Errors
    ///
    /// Returns [`NliError::Inference`] if tokenization or model inference
    /// fails for any label.
    pub fn classify(
        &self,
        text: &str,
        labels: &[&str],
        hypothesis_template: &str,
    ) -> Result<Vec<ScoredLabel>, NliError> {
        if labels.is_empty() {
            return Ok(Vec::new());
        }

        let mut scored: Vec<ScoredLabel> = labels
            .iter()
            .map(|label| {
                let hypothesis = hypothesis_template.replace("{}", label);
                let score = self.entailment_score(text, &hypothesis)?;
                Ok(ScoredLabel {
                    label: (*label).to_string(),
                    score,
                })
            })
            .collect::<Result<Vec<_>, NliError>>()?;

        scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        Ok(scored)
    }

    /// Computes the entailment probability for one premise-hypothesis pair.
    fn entailment_score(&self, premise: &str, hypothesis: &str) -> Result<f32, NliError> {
        let encoding = {
            let tokenizer = self
                .tokenizer
                .lock()
                .map_err(|e| NliError::Inference(format!("tokenizer lock poisoned: {e}")))?;
            // The tokenizer pairs the inputs as [CLS] premise [SEP] hypothesis [SEP].
            tokenizer
                .encode((premise, hypothesis), true)
                .map_err(|e| NliError::Inference(format!("tokenization failed: {e}")))?
        };

        let input_ids: Vec<i64> = encoding.get_ids().iter().map(|&id| i64::from(id)).collect();
        let attention_mask: Vec<i64> = encoding.get_attention_mask().iter().map(|&m| i64::from(m)).collect();
        let shape = [1_usize, input_ids.len()];

        let input_ids_tensor = Tensor::from_array((shape, input_ids))
            .map_err(|e| NliError::Inference(format!("failed to create input_ids tensor: {e}")))?;
        let attention_mask_tensor = Tensor::from_array((shape, attention_mask))
            .map_err(|e| NliError::Inference(format!("failed to create attention_mask tensor: {e}")))?;

        let mut session = self
            .session
            .lock()
            .map_err(|e| NliError::Inference(format!("session lock poisoned: {e}")))?;

        // DeBERTa-v2 takes only input_ids + attention_mask (no token_type_ids).
        let outputs = session
            .run(ort::inputs![input_ids_tensor, attention_mask_tensor])
            .map_err(|e| NliError::Inference(format!("model inference failed: {e}")))?;

        // Output shape is [1, 2] — [entailment, not_entailment]. We only need
        // the data slice, not the shape.
        let (_shape, logits) = outputs[0]
            .try_extract_tensor::<f32>()
            .map_err(|e| NliError::Inference(format!("failed to extract logits: {e}")))?;

        if logits.len() < 2 {
            return Err(NliError::Inference(format!(
                "expected at least 2 logits, got {}",
                logits.len()
            )));
        }

        Ok(softmax(logits)[ENTAILMENT_IDX])
    }
}

/// Downloads the model and tokenizer from HuggingFace, caching them locally.
fn download_model_files(
    repo: &str,
    model_file: &str,
    tokenizer_file: &str,
) -> Result<(std::path::PathBuf, std::path::PathBuf), NliError> {
    let api = hf_hub::api::sync::Api::new().map_err(|e| NliError::Download(e.to_string()))?;
    let repo = api.model(repo.to_string());

    let model_path = repo
        .get(model_file)
        .map_err(|e| NliError::Download(format!("failed to download {model_file}: {e}")))?;
    let tokenizer_path = repo
        .get(tokenizer_file)
        .map_err(|e| NliError::Download(format!("failed to download {tokenizer_file}: {e}")))?;

    Ok((model_path, tokenizer_path))
}

/// Builds an ONNX session on CPU.
///
/// Without the `cuda` feature this is the only path: the CPU provider is always
/// available, so there is no provider to negotiate.
#[cfg(not(feature = "cuda"))]
fn create_session(model_path: &std::path::Path) -> Result<(Session, ExecutionProvider), NliError> {
    let session = build_cpu_session(model_path)
        .map_err(|e| NliError::ModelLoad(format!("failed to initialize NLI session on CPU: {e}")))?;
    Ok((session, ExecutionProvider::Cpu))
}

/// Builds a CPU ONNX session from `model_path`.
fn build_cpu_session(model_path: &std::path::Path) -> Result<Session, String> {
    // A single intra-op thread: classification runs one short sequence per
    // call, so extra threads add scheduling overhead without speedup. rc.12's
    // builder methods return `Error<SessionBuilder>` (carrying the builder for
    // recovery), so `?` is used to coerce into a uniform error rather than
    // `.and_then`, whose error types would not unify with `commit_from_file`.
    build_cpu_session_inner(model_path).map_err(|e| e.to_string())
}

/// Inner CPU session builder using `?`-chaining over `ort::Result`.
fn build_cpu_session_inner(model_path: &std::path::Path) -> ort::Result<Session> {
    let session = Session::builder()?
        .with_intra_threads(1)?
        .commit_from_file(model_path)?;
    Ok(session)
}

/// Builds an ONNX session honoring `NLI_EXECUTION_PROVIDER` (CUDA build).
///
/// Reads the provider preference from the environment (`auto` | `cuda` | `cpu`,
/// defaulting to `auto`). `auto` attempts CUDA and falls back to CPU; an
/// explicit `cuda` or `cpu` uses only that provider.
#[cfg(feature = "cuda")]
fn create_session(model_path: &std::path::Path) -> Result<(Session, ExecutionProvider), NliError> {
    match ExecutionProviderPreference::from_env()? {
        ExecutionProviderPreference::Auto => create_auto_session(model_path),
        ExecutionProviderPreference::Cuda => build_gpu_session(model_path, ExecutionProvider::Cuda)
            .map(|session| (session, ExecutionProvider::Cuda))
            .map_err(|e| NliError::ModelLoad(format!("failed to initialize NLI session on CUDA: {e}"))),
        ExecutionProviderPreference::Cpu => build_cpu_session(model_path)
            .map(|session| (session, ExecutionProvider::Cpu))
            .map_err(|e| NliError::ModelLoad(format!("failed to initialize NLI session on CPU: {e}"))),
    }
}

/// Attempts CUDA, falling back to CPU on failure (CUDA build).
#[cfg(feature = "cuda")]
fn create_auto_session(model_path: &std::path::Path) -> Result<(Session, ExecutionProvider), NliError> {
    match build_gpu_session(model_path, ExecutionProvider::Cuda) {
        Ok(session) => Ok((session, ExecutionProvider::Cuda)),
        Err(cuda_err) => {
            let session = build_cpu_session(model_path).map_err(|cpu_err| {
                NliError::ModelLoad(format!(
                    "failed to initialize NLI session; CUDA error: {cuda_err}; CPU fallback error: {cpu_err}"
                ))
            })?;
            tracing::event!(
                name: "memoir.nli.cuda_fallback",
                tracing::Level::WARN,
                error = %cuda_err,
                "CUDA init failed; falling back to CPU",
            );
            Ok((session, ExecutionProvider::Cpu))
        }
    }
}

/// Builds a session bound to a specific execution provider (CUDA build).
#[cfg(feature = "cuda")]
fn build_gpu_session(model_path: &std::path::Path, provider: ExecutionProvider) -> Result<Session, String> {
    build_gpu_session_inner(model_path, provider).map_err(|e| e.to_string())
}

/// Inner provider-bound session builder using `?`-chaining (CUDA build).
#[cfg(feature = "cuda")]
fn build_gpu_session_inner(model_path: &std::path::Path, provider: ExecutionProvider) -> ort::Result<Session> {
    let dispatch = match provider {
        ExecutionProvider::Cuda => ort::ep::CUDA::default().build().error_on_failure(),
        ExecutionProvider::Cpu => ort::ep::CPU::default().build().error_on_failure(),
    };
    let session = Session::builder()?
        .with_execution_providers([dispatch])?
        .with_intra_threads(1)?
        .commit_from_file(model_path)?;
    Ok(session)
}

/// Execution-provider preference parsed from `NLI_EXECUTION_PROVIDER`.
#[cfg(feature = "cuda")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExecutionProviderPreference {
    Auto,
    Cuda,
    Cpu,
}

#[cfg(feature = "cuda")]
impl ExecutionProviderPreference {
    fn from_env() -> Result<Self, NliError> {
        match std::env::var("NLI_EXECUTION_PROVIDER") {
            Ok(value) => Self::parse(&value).map_err(|invalid| {
                NliError::ModelLoad(format!(
                    "invalid NLI_EXECUTION_PROVIDER `{invalid}`; expected one of: auto, cuda, cpu"
                ))
            }),
            Err(std::env::VarError::NotPresent) => Ok(Self::Auto),
            Err(e) => Err(NliError::ModelLoad(format!(
                "failed to read NLI_EXECUTION_PROVIDER: {e}"
            ))),
        }
    }

    fn parse(value: &str) -> Result<Self, &str> {
        match value.trim().to_ascii_lowercase().as_str() {
            "auto" => Ok(Self::Auto),
            "cuda" => Ok(Self::Cuda),
            "cpu" => Ok(Self::Cpu),
            _ => Err(value),
        }
    }
}

/// Softmax over a slice of logits.
fn softmax(logits: &[f32]) -> Vec<f32> {
    let max = logits.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let exps: Vec<f32> = logits.iter().map(|&x| (x - max).exp()).collect();
    let sum: f32 = exps.iter().sum();
    exps.iter().map(|&e| e / sum).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_compute_softmax_correctly() {
        let logits = [2.0, 1.0, 0.1];
        let probs = softmax(&logits);

        assert!((probs.iter().sum::<f32>() - 1.0).abs() < 1e-5);
        assert!(probs[0] > probs[1]);
        assert!(probs[1] > probs[2]);
    }

    #[test]
    fn should_handle_softmax_with_large_values() {
        // Shifting by the max keeps exp() from overflowing on large logits.
        let logits = [1000.0, 1.0, 0.1];
        let probs = softmax(&logits);

        assert!((probs.iter().sum::<f32>() - 1.0).abs() < 1e-5);
        assert!(probs[0] > 0.99);
    }

    #[test]
    fn should_report_cpu_provider_ort_name() {
        assert_eq!(ExecutionProvider::Cpu.ort_name(), "CPUExecutionProvider");
    }

    #[test]
    fn should_default_nli_config_to_the_shipped_moritzlaurer_model() {
        let NliConfig::HuggingFace {
            repo,
            model_file,
            tokenizer_file,
        } = NliConfig::default();
        assert_eq!(repo, "MoritzLaurer/deberta-v3-xsmall-zeroshot-v1.1-all-33");
        assert_eq!(model_file, "onnx/model_quantized.onnx");
        assert_eq!(tokenizer_file, "tokenizer.json");
    }

    #[test]
    fn should_build_nli_config_from_huggingface_constructor() {
        let config = NliConfig::huggingface("org/model", "m.onnx", "tok.json");
        assert_eq!(
            config,
            NliConfig::HuggingFace {
                repo: "org/model".to_string(),
                model_file: "m.onnx".to_string(),
                tokenizer_file: "tok.json".to_string(),
            }
        );
    }

    #[cfg(feature = "cuda")]
    #[test]
    fn should_parse_auto_execution_provider_preference() {
        assert_eq!(
            ExecutionProviderPreference::parse("auto"),
            Ok(ExecutionProviderPreference::Auto)
        );
    }

    #[cfg(feature = "cuda")]
    #[test]
    fn should_parse_cpu_execution_provider_preference_case_insensitively() {
        assert_eq!(
            ExecutionProviderPreference::parse("CPU"),
            Ok(ExecutionProviderPreference::Cpu)
        );
    }

    #[cfg(feature = "cuda")]
    #[test]
    fn should_parse_cuda_execution_provider_preference_with_whitespace() {
        assert_eq!(
            ExecutionProviderPreference::parse(" cuda "),
            Ok(ExecutionProviderPreference::Cuda)
        );
    }

    #[cfg(feature = "cuda")]
    #[test]
    fn should_reject_unknown_execution_provider_preference() {
        assert_eq!(ExecutionProviderPreference::parse("metal"), Err("metal"));
    }
}
