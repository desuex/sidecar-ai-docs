use crate::adapter::{LanguageAdapter, RawRef, RawSymbol};
use sidecar_types::Language;

/// TypeScript/JavaScript language adapter using Tree-sitter.
pub struct TypeScriptAdapter {
    // Tree-sitter parser will be initialized here in M1.
}

impl TypeScriptAdapter {
    pub fn new() -> Self {
        TypeScriptAdapter {}
    }
}

impl Default for TypeScriptAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageAdapter for TypeScriptAdapter {
    fn language(&self) -> Language {
        Language::TypeScript
    }

    fn parse_symbols(&self, _source: &[u8]) -> Vec<RawSymbol> {
        // TODO(M1): Tree-sitter symbol extraction
        Vec::new()
    }

    fn parse_refs(&self, _source: &[u8]) -> Vec<RawRef> {
        // TODO(M2): Tree-sitter reference extraction
        Vec::new()
    }
}
