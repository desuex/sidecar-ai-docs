use sidecar_types::{Language, Range, RefKind, SymbolKind, Visibility};

/// Pre-UID symbol data extracted from AST.
#[derive(Debug, Clone)]
pub struct RawSymbol {
    pub name: String,
    pub qualified_name: String,
    pub kind: SymbolKind,
    pub visibility: Visibility,
    pub range: Range,
    /// Input bytes for structural fingerprint computation.
    pub fingerprint_input: String,
}

/// Raw reference extracted from AST (before UID resolution).
#[derive(Debug, Clone)]
pub struct RawRef {
    pub from_qualified_name: String,
    pub to_name: String,
    pub range: Range,
    pub ref_kind: RefKind,
}

/// Trait for language-specific AST parsing.
pub trait LanguageAdapter: Send + Sync {
    fn language(&self) -> Language;
    fn parse_symbols(&self, source: &[u8]) -> Vec<RawSymbol>;
    fn parse_refs(&self, source: &[u8]) -> Vec<RawRef>;
}
