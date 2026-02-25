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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Symbol;
    use sidecar_types::{Fingerprint, Range, SymbolKind, Uid, Visibility};

    fn symbol(name: &str, qualified_name: &str) -> Symbol {
        Symbol {
            uid: "sym:ts:src/sample:Sample:abcd1234".parse::<Uid>().unwrap(),
            file_uid: "file:src/sample.ts".parse::<Uid>().unwrap(),
            kind: SymbolKind::Class,
            qualified_name: qualified_name.to_owned(),
            name: name.to_owned(),
            visibility: Visibility::Public,
            fingerprint: Fingerprint::from_hex("abcd1234".to_owned()),
            range: Range { start: 0, end: 10 },
        }
    }

    #[test]
    fn score_exact_match() {
        let s = symbol("CartService", "CartService");
        assert_eq!(score_symbol("CartService", &s), 1.0);
    }

    #[test]
    fn score_prefix_match() {
        let s = symbol("CartService", "CartService");
        assert_eq!(score_symbol("Cart", &s), 0.7);
    }

    #[test]
    fn score_qualified_name_contains_query() {
        let s = symbol("Service", "CartService.calculateTotal");
        assert_eq!(score_symbol("calculate", &s), 0.5);
    }

    #[test]
    fn score_non_match() {
        let s = symbol("Service", "CartService.calculateTotal");
        assert_eq!(score_symbol("inventory", &s), 0.0);
    }
}
