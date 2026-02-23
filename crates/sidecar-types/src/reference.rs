use serde::{Deserialize, Serialize};

/// Kind of reference between symbols.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefKind {
    Call,
    Import,
    TypeRef,
    Inherit,
    Unknown,
}
