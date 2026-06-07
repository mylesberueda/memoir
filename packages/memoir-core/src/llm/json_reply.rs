//! Locating a JSON object inside a noisy LLM text reply.
//!
//! Small models wrap their JSON in markdown fences, leading reasoning, or
//! trailing notes. [`locate_json_object`] finds the first balanced `{...}`
//! object in such a reply so a serde deserializer can take it from there. It is
//! the shared front-half of every "parse a structured LLM reply" path
//! ([`super::extraction`], [`crate::graph::extraction`]); the back-half (the
//! actual deserialize into a typed shape) stays with each caller.

/// Returns the first balanced `{...}` slice within `text`.
///
/// Handles three common cases:
/// - **Bare JSON**: returns the input as-is (with markdown fences stripped if present).
/// - **Markdown-fenced JSON**: strips ``` or ```json fences before scanning.
/// - **Prose + JSON**: scans for the first `{` and finds the matching `}`,
///   counting nested braces and ignoring braces inside string literals.
pub(crate) fn locate_json_object(text: &str) -> Option<&str> {
    let body = strip_markdown_fences(text);

    let bytes = body.as_bytes();
    let start = body.find('{')?;

    let mut depth = 0i32;
    let mut in_string = false;
    let mut escape = false;

    for (i, &b) in bytes.iter().enumerate().skip(start) {
        if escape {
            escape = false;
            continue;
        }
        if in_string {
            match b {
                b'\\' => escape = true,
                b'"' => in_string = false,
                _ => {}
            }
            continue;
        }
        match b {
            b'"' => in_string = true,
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&body[start..=i]);
                }
            }
            _ => {}
        }
    }

    None
}

/// Removes leading/trailing markdown fence markers (`\`\`\`` or `\`\`\`json`).
///
/// Returns the original string if no fences are present.
fn strip_markdown_fences(text: &str) -> &str {
    let trimmed = text.trim();

    let after_open = if let Some(rest) = trimmed.strip_prefix("```json") {
        rest.trim_start_matches('\n').trim_start_matches('\r')
    } else if let Some(rest) = trimmed.strip_prefix("```") {
        rest.trim_start_matches('\n').trim_start_matches('\r')
    } else {
        return trimmed;
    };

    match after_open.rsplit_once("```") {
        Some((before, _after)) => before.trim_end(),
        None => after_open,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_locate_json_object_handle_strings_with_braces() {
        let input = r#"prose {"a": "has } in it", "b": 1} more prose"#;
        let located = locate_json_object(input).unwrap();
        assert_eq!(located, r#"{"a": "has } in it", "b": 1}"#);
    }

    #[test]
    fn should_locate_json_object_handle_escaped_quotes() {
        let input = r#"prose {"a": "has \" escaped"} more"#;
        let located = locate_json_object(input).unwrap();
        assert_eq!(located, r#"{"a": "has \" escaped"}"#);
    }

    #[test]
    fn should_strip_json_language_fence() {
        let input = "```json\n{\"a\":1}\n```";
        let located = locate_json_object(input).unwrap();
        assert_eq!(located, "{\"a\":1}");
    }

    #[test]
    fn should_strip_bare_fence_without_language() {
        let input = "```\n{\"a\":1}\n```";
        let located = locate_json_object(input).unwrap();
        assert_eq!(located, "{\"a\":1}");
    }

    #[test]
    fn should_return_none_when_no_object_present() {
        assert_eq!(locate_json_object("no json here"), None);
    }
}
