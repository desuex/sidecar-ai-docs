use crate::model::Symbol;

/// Deterministic relevance score for a symbol against a query string.
///
/// Scoring rules (versioned: ranking_v1):
/// - Exact name match: 1.0
/// - Name prefix match: 0.7
/// - Qualified name contains query: 0.5
/// - Otherwise: 0.0
///
/// Tie-breaker: lexicographic order by UID (deterministic).
pub fn score_symbol(query: &str, symbol: &Symbol) -> f64 {
    let q = query.to_lowercase();
    let name = symbol.name.to_lowercase();
    let qname = symbol.qualified_name.to_lowercase();

    if name == q {
        1.0
    } else if name.starts_with(&q) {
        0.7
    } else if qname.contains(&q) {
        0.5
    } else {
        0.0
    }
}
