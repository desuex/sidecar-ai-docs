use sidecar_types::{Fingerprint, Language, Uid};

/// Generate a deterministic symbol UID.
///
/// Format: `sym:<language>:<module_path>:<qualified_name>:<struct_hash>`
///
/// The struct_hash is the first 8 hex chars of the fingerprint.
pub fn generate_uid(
    language: Language,
    module_path: &str,
    qualified_name: &str,
    fingerprint: &Fingerprint,
) -> Result<Uid, sidecar_types::uid::UidParseError> {
    let hash_prefix = &fingerprint.as_str()[..std::cmp::min(8, fingerprint.as_str().len())];
    let raw = format!(
        "sym:{}:{}:{}:{}",
        language.code(),
        module_path,
        qualified_name,
        hash_prefix,
    );
    raw.parse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fingerprint::compute_fingerprint;

    #[test]
    fn uid_generation_is_deterministic() {
        let fp = compute_fingerprint(b"function_declaration\nname: foo\nparent: Bar");
        let uid1 = generate_uid(Language::TypeScript, "src/bar", "Bar.foo", &fp).unwrap();
        let uid2 = generate_uid(Language::TypeScript, "src/bar", "Bar.foo", &fp).unwrap();
        assert_eq!(uid1, uid2);
    }

    #[test]
    fn different_input_different_uid() {
        let fp_a = compute_fingerprint(b"function_declaration\nname: foo");
        let fp_b = compute_fingerprint(b"function_declaration\nname: bar");
        let uid_a = generate_uid(Language::TypeScript, "src/a", "A.foo", &fp_a).unwrap();
        let uid_b = generate_uid(Language::TypeScript, "src/a", "A.bar", &fp_b).unwrap();
        assert_ne!(uid_a, uid_b);
    }
}
