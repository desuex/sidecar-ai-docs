use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Validated UID for code symbols.
///
/// Format: `sym:<language>:<module_path>:<qualified_name>:<struct_hash>`
/// Example: `sym:ts:src/services/cart:CartService.calculateTotal:ab12cd34`
///
/// Also supports: `file:<path>`, `module:<path>`, `doc:<slug>`, `concept:<slug>`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Uid(String);

// Symbol UID: sym:<lang>:<module_path>:<qualified_name>:<hex_hash>
static SYM_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^sym:[a-z]{1,10}:[a-zA-Z0-9_./-]+:[a-zA-Z0-9_.$<>\[\]-]+:[a-f0-9]{4,64}$").unwrap()
});

// File UID: file:<normalized_path>
static FILE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^file:[a-zA-Z0-9_./-]+$").unwrap());

// Module UID: module:<path_or_namespace>
static MODULE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^module:[a-zA-Z0-9_./-]+$").unwrap());

// Doc UID: doc:<slug>
static DOC_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^doc:[a-zA-Z0-9_-]+$").unwrap());

// Concept UID: concept:<slug>
static CONCEPT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^concept:[a-zA-Z0-9_-]+$").unwrap());

impl Uid {
    /// Returns the raw UID string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the UID prefix (e.g., "sym", "file", "doc").
    pub fn kind(&self) -> &str {
        self.0.split(':').next().unwrap_or("")
    }
}

impl FromStr for Uid {
    type Err = UidParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("..") {
            return Err(UidParseError::PathTraversal);
        }

        let valid = SYM_RE.is_match(s)
            || FILE_RE.is_match(s)
            || MODULE_RE.is_match(s)
            || DOC_RE.is_match(s)
            || CONCEPT_RE.is_match(s);

        if valid {
            Ok(Uid(s.to_owned()))
        } else {
            Err(UidParseError::InvalidFormat(s.to_owned()))
        }
    }
}

impl fmt::Display for Uid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum UidParseError {
    #[error("invalid UID format: {0}")]
    InvalidFormat(String),
    #[error("UID contains path traversal")]
    PathTraversal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_symbol_uid() {
        let uid: Uid = "sym:ts:src/services/cart:CartService.calculateTotal:ab12cd34"
            .parse()
            .unwrap();
        assert_eq!(uid.kind(), "sym");
    }

    #[test]
    fn valid_file_uid() {
        let uid: Uid = "file:src/services/cart.ts".parse().unwrap();
        assert_eq!(uid.kind(), "file");
    }

    #[test]
    fn valid_doc_uid() {
        let uid: Uid = "doc:cart-calc-overview".parse().unwrap();
        assert_eq!(uid.kind(), "doc");
    }

    #[test]
    fn valid_concept_uid() {
        let uid: Uid = "concept:pricing-engine".parse().unwrap();
        assert_eq!(uid.kind(), "concept");
    }

    #[test]
    fn valid_module_uid() {
        let uid: Uid = "module:src/services".parse().unwrap();
        assert_eq!(uid.kind(), "module");
    }

    #[test]
    fn rejects_empty() {
        assert!("".parse::<Uid>().is_err());
    }

    #[test]
    fn rejects_path_traversal() {
        assert!("file:../etc/passwd".parse::<Uid>().is_err());
        assert!("sym:ts:../escape:Foo:abcd1234".parse::<Uid>().is_err());
    }

    #[test]
    fn rejects_unknown_prefix() {
        assert!("unknown:something".parse::<Uid>().is_err());
    }

    #[test]
    fn rejects_spaces() {
        assert!("sym:ts:src/a:Foo Bar:abcd1234".parse::<Uid>().is_err());
    }

    #[test]
    fn ordering_is_deterministic() {
        let a: Uid = "sym:ts:a:A:0001".parse().unwrap();
        let b: Uid = "sym:ts:b:B:0002".parse().unwrap();
        assert!(a < b);
    }

    #[test]
    fn serde_roundtrip() {
        let uid: Uid = "sym:ts:src/cart:Cart.total:abcd1234".parse().unwrap();
        let json = serde_json::to_string(&uid).unwrap();
        let back: Uid = serde_json::from_str(&json).unwrap();
        assert_eq!(uid, back);
    }
}
