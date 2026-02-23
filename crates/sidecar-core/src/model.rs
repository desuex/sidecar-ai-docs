use serde::Serialize;
use sidecar_types::{
    ContentHash, Fingerprint, Language, PathRel, Range, RefKind, SymbolKind, Uid, Visibility,
};

/// Indexed file record.
#[derive(Debug, Clone, Serialize)]
pub struct FileRecord {
    pub file_uid: Uid,
    pub path: PathRel,
    pub language: Language,
    pub content_hash: ContentHash,
    pub last_indexed_at: String,
}

/// Indexed symbol record.
#[derive(Debug, Clone, Serialize)]
pub struct Symbol {
    pub uid: Uid,
    pub file_uid: Uid,
    pub kind: SymbolKind,
    pub qualified_name: String,
    pub name: String,
    pub visibility: Visibility,
    pub fingerprint: Fingerprint,
    pub range: Range,
}

/// Reference between symbols.
#[derive(Debug, Clone, Serialize)]
pub struct Reference {
    pub from_uid: Uid,
    pub to_uid: Uid,
    pub file_uid: Uid,
    pub range: Range,
    pub ref_kind: RefKind,
}

/// Sidecar documentation record.
#[derive(Debug, Clone, Serialize)]
pub struct DocRecord {
    pub doc_uid: Uid,
    pub target_uid: Uid,
    pub path: PathRel,
    pub summary_cache: Option<String>,
    pub updated_at: String,
}
