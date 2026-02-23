//! Integration test: index a TypeScript fixture project, then query it.

use std::path::PathBuf;

use sidecar_core::indexer::index_project;
use sidecar_core::query::SearchQuery;
use sidecar_core::Repository;
use sidecar_parsing::TypeScriptAdapter;
use sidecar_storage::SqliteRepository;
use sidecar_types::{Limit, Offset};

fn fixture_root() -> PathBuf {
    // Navigate from crates/sidecar-core/ up to workspace root, then into fixtures/
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/ts-sample")
        .canonicalize()
        .expect("fixtures/ts-sample must exist")
}

fn index_fixture() -> (SqliteRepository, sidecar_core::indexer::IndexResult) {
    let repo = SqliteRepository::open_in_memory().expect("open in-memory DB");
    let ts = TypeScriptAdapter::new();
    let adapters: Vec<&dyn sidecar_parsing::LanguageAdapter> = vec![&ts];
    let result = index_project(&fixture_root(), &repo, &adapters).expect("index_project");
    (repo, result)
}

#[test]
fn indexes_all_fixture_files() {
    let (_repo, result) = index_fixture();
    assert_eq!(
        result.files_indexed, 3,
        "should index cart.ts, utils.ts, types.ts"
    );
    assert!(result.symbols_extracted > 0, "should extract symbols");
}

#[test]
fn search_cart_service() {
    let (repo, _) = index_fixture();
    let result = repo
        .search_symbols(&SearchQuery {
            query: "CartService".to_string(),
            limit: Limit::default(),
            offset: Offset::default(),
        })
        .expect("search");

    assert!(!result.results.is_empty(), "should find CartService");
    let names: Vec<&str> = result.results.iter().map(|s| s.name.as_str()).collect();
    assert!(
        names.contains(&"CartService"),
        "results should contain CartService: {names:?}"
    );
}

#[test]
fn search_results_snapshot() {
    let (repo, _) = index_fixture();
    let result = repo
        .search_symbols(&SearchQuery {
            query: "Cart".to_string(),
            limit: Limit::default(),
            offset: Offset::default(),
        })
        .expect("search");

    // Snapshot the symbol names + kinds for stability
    let summary: Vec<String> = result
        .results
        .iter()
        .map(|s| format!("{} ({:?})", s.qualified_name, s.kind))
        .collect();
    insta::assert_yaml_snapshot!(summary);
}

#[test]
fn get_symbol_by_uid() {
    let (repo, _) = index_fixture();
    let result = repo
        .search_symbols(&SearchQuery {
            query: "CartService".to_string(),
            limit: Limit::new(1).unwrap(),
            offset: Offset::default(),
        })
        .expect("search");

    let sym = &result.results[0];
    let found = repo
        .get_symbol(&sym.uid)
        .expect("get_symbol")
        .expect("symbol should exist");
    assert_eq!(found.uid, sym.uid);
    assert_eq!(found.name, sym.name);
}

#[test]
fn indexing_is_idempotent() {
    let repo = SqliteRepository::open_in_memory().expect("open in-memory DB");
    let ts = TypeScriptAdapter::new();
    let adapters: Vec<&dyn sidecar_parsing::LanguageAdapter> = vec![&ts];
    let root = fixture_root();

    let r1 = index_project(&root, &repo, &adapters).expect("first index");
    assert_eq!(r1.files_indexed, 3);

    // Second run: files unchanged → all skipped
    let r2 = index_project(&root, &repo, &adapters).expect("second index");
    assert_eq!(r2.files_indexed, 0, "unchanged files should be skipped");
    assert_eq!(r2.files_skipped, 3);
}

#[test]
fn deterministic_uids() {
    // Index twice in separate DBs → same UIDs
    let (repo1, _) = index_fixture();
    let (repo2, _) = index_fixture();

    let r1 = repo1
        .search_symbols(&SearchQuery {
            query: "CartService".to_string(),
            limit: Limit::default(),
            offset: Offset::default(),
        })
        .expect("search1");
    let r2 = repo2
        .search_symbols(&SearchQuery {
            query: "CartService".to_string(),
            limit: Limit::default(),
            offset: Offset::default(),
        })
        .expect("search2");

    let uids1: Vec<String> = r1.results.iter().map(|s| s.uid.to_string()).collect();
    let uids2: Vec<String> = r2.results.iter().map(|s| s.uid.to_string()).collect();
    assert_eq!(uids1, uids2, "UIDs must be deterministic across runs");
}
