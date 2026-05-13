use kreuzberg::{ChunkingConfig, ExtractionConfig, ExtractionResult, KreuzbergError, extract_bytes};

/// Default chunk size in characters (~512 tokens).
const CHUNK_MAX_CHARS: usize = 2000;
/// Overlap between consecutive chunks in characters.
const CHUNK_OVERLAP: usize = 200;

/// Extract text and produce chunks from raw file bytes.
///
/// Uses kreuzberg for format detection + extraction + chunking in one pass.
/// Supports 75+ formats including PDF, Office, HTML, plain text, etc.
pub(crate) async fn extract_and_chunk(bytes: &[u8], mime_type: &str) -> Result<ExtractionResult, KreuzbergError> {
    let config = ExtractionConfig {
        chunking: Some(ChunkingConfig {
            max_characters: CHUNK_MAX_CHARS,
            overlap: CHUNK_OVERLAP,
            ..Default::default()
        }),
        ..Default::default()
    };

    extract_bytes(bytes, mime_type, &config).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_extract_plain_text() {
        let content = b"This is a test document with enough text to be processed.";
        let result = extract_and_chunk(content, "text/plain").await;

        assert!(result.is_ok(), "extraction failed: {:?}", result.err());
        let result = result.unwrap();
        assert!(!result.content.is_empty(), "extracted content should not be empty");
    }

    #[tokio::test]
    async fn should_return_chunks_for_long_text() {
        // Generate text longer than CHUNK_MAX_CHARS (2000) to trigger chunking
        let content = "The quick brown fox jumps over the lazy dog. ".repeat(100);
        let result = extract_and_chunk(content.as_bytes(), "text/plain").await;

        assert!(result.is_ok(), "extraction failed: {:?}", result.err());
        let result = result.unwrap();
        assert!(!result.content.is_empty());
        assert!(result.chunks.is_some(), "should produce chunks for long text");
        assert!(
            result.chunks.as_ref().unwrap().len() > 1,
            "should produce multiple chunks for text > {} chars",
            CHUNK_MAX_CHARS
        );
    }

    #[tokio::test]
    async fn should_handle_empty_bytes() {
        let result = extract_and_chunk(b"", "text/plain").await;
        // kreuzberg may succeed with empty content or return an error — either is valid
        // but it should not panic
        if let Ok(result) = result {
            assert!(result.content.is_empty(), "empty input should produce empty content");
        }
    }
}
