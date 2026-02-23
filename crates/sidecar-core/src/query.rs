use serde::Serialize;
use sidecar_types::{Limit, Offset};

use crate::model::{Reference, Symbol};

/// Search query parameters.
pub struct SearchQuery {
    pub query: String,
    pub limit: Limit,
    pub offset: Offset,
}

/// Search result with truncation flag.
#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub results: Vec<Symbol>,
    pub truncated: bool,
}

/// References query parameters.
pub struct RefsQuery {
    pub limit: Limit,
    pub offset: Offset,
}

/// References result with total count and truncation flag.
#[derive(Debug, Serialize)]
pub struct RefsResult {
    pub total: u32,
    pub results: Vec<Reference>,
    pub truncated: bool,
}
