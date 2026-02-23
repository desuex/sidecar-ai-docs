use sidecar_types::{ContentHash, Fingerprint};

/// Compute structural fingerprint from normalized AST bytes.
///
/// Uses BLAKE3. Returns hex-encoded truncated hash.
pub fn compute_fingerprint(normalized_input: &[u8]) -> Fingerprint {
    let hash = blake3::hash(normalized_input);
    Fingerprint::from_hex(hash.to_hex().to_string())
}

/// Compute content hash for a file (full BLAKE3).
pub fn compute_content_hash(content: &[u8]) -> ContentHash {
    let hash = blake3::hash(content);
    ContentHash::from_hex(hash.to_hex().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fingerprint_is_deterministic() {
        let a = compute_fingerprint(b"function foo() {}");
        let b = compute_fingerprint(b"function foo() {}");
        assert_eq!(a, b);
    }

    #[test]
    fn different_input_different_fingerprint() {
        let a = compute_fingerprint(b"function foo() {}");
        let b = compute_fingerprint(b"function bar() {}");
        assert_ne!(a, b);
    }

    #[test]
    fn content_hash_deterministic() {
        let a = compute_content_hash(b"const x = 1;");
        let b = compute_content_hash(b"const x = 1;");
        assert_eq!(a, b);
    }
}
