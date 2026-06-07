//! Cosine similarity over float vectors, shared by the graph resolvers.
//!
//! The entity resolver ([`super::resolve`]) and the synthesizer
//! ([`super::synthesis`]) both score candidate matches by cosine similarity;
//! this is their one shared implementation.

/// Returns the cosine similarity of `a` and `b`, or `None` if undefined.
///
/// Undefined when the vectors differ in length or either has zero magnitude.
/// Callers treat `None` as "not a match" rather than a hard error, so one
/// malformed vector does not fail the whole comparison.
pub(super) fn cosine_similarity(a: &[f32], b: &[f32]) -> Option<f32> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_compute_cosine_for_identical_vectors() {
        let similarity = cosine_similarity(&[1.0, 0.0], &[1.0, 0.0]).unwrap();
        assert!((similarity - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn should_return_none_for_mismatched_lengths() {
        assert_eq!(cosine_similarity(&[1.0, 0.0], &[1.0]), None);
    }

    #[test]
    fn should_return_none_for_zero_magnitude() {
        assert_eq!(cosine_similarity(&[0.0, 0.0], &[1.0, 0.0]), None);
    }
}
