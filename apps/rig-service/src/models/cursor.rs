use base64::Engine as _;
use serde::{Deserialize, Serialize};
use std::fmt;
use tonic::Status;

/// Opaque cursor token for keyset pagination.
///
/// Encodes any serializable tuple as a base64 JSON string.
/// The client treats it as an opaque token — never parses it.
///
/// # Usage
///
/// ```ignore
/// type ShareCursor = Cursor<(DateTime, i64)>;
///
/// // Encode
/// let token = ShareCursor::new((last.created_at, last.id)).to_string();
///
/// // Decode
/// let c = ShareCursor::try_from(token.as_str())?;
/// cursor.after(c.into_inner());
/// ```
pub(crate) struct Cursor<T>(T);

impl<T> Cursor<T> {
    pub fn new(inner: T) -> Self {
        Self(inner)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Serialize> fmt::Display for Cursor<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string(&self.0).map_err(|_| fmt::Error)?;
        let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(json);
        f.write_str(&encoded)
    }
}

impl<T: for<'de> Deserialize<'de>> TryFrom<&str> for Cursor<T> {
    type Error = Status;

    fn try_from(s: &str) -> Result<Self, Status> {
        let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(s)
            .map_err(|_| Status::invalid_argument("Invalid cursor"))?;
        let inner: T = serde_json::from_slice(&bytes).map_err(|_| Status::invalid_argument("Invalid cursor"))?;
        Ok(Self(inner))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestCursor = Cursor<(String, i64)>;

    #[test]
    fn should_roundtrip_cursor() {
        let original = TestCursor::new(("hello".to_string(), 42));
        let encoded = original.to_string();
        let decoded = TestCursor::try_from(encoded.as_str()).unwrap();
        let (s, n) = decoded.into_inner();
        assert_eq!(s, "hello");
        assert_eq!(n, 42);
    }

    #[test]
    fn should_reject_invalid_base64() {
        let result = TestCursor::try_from("not-valid-!!!!");
        assert!(result.is_err());
    }

    #[test]
    fn should_reject_wrong_shape() {
        let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(serde_json::json!([1, 2, 3]).to_string());
        let result = TestCursor::try_from(encoded.as_str());
        assert!(result.is_err());
    }
}
