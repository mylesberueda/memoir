use base64::Engine as _;
use serde::{Deserialize, Serialize};
use std::fmt;
use tonic::Status;

/// Opaque cursor token for keyset pagination.
///
/// Encodes any serializable tuple as a base64 JSON string.
/// The client treats it as an opaque token — never parses it.
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
