use serde::{Deserialize, Serialize};
use std::fmt;

/// BLAKE3 content hash of a file.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ContentHash(String);

impl ContentHash {
    /// Create from a hex-encoded hash string.
    pub fn from_hex(hex: String) -> Self {
        ContentHash(hex)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ContentHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Structural fingerprint of a symbol (truncated hash).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Fingerprint(String);

impl Fingerprint {
    /// Create from a hex-encoded fingerprint string.
    pub fn from_hex(hex: String) -> Self {
        Fingerprint(hex)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{ContentHash, Fingerprint};

    #[test]
    fn content_hash_accessors() {
        let hash = ContentHash::from_hex("abc123".to_owned());
        assert_eq!(hash.as_str(), "abc123");
        assert_eq!(hash.to_string(), "abc123");
    }

    #[test]
    fn fingerprint_accessors() {
        let fingerprint = Fingerprint::from_hex("def456".to_owned());
        assert_eq!(fingerprint.as_str(), "def456");
        assert_eq!(fingerprint.to_string(), "def456");
    }
}
