use std::path::Path;

use sidecar_core::query::SearchQuery;
use sidecar_core::Repository;
use sidecar_storage::SqliteRepository;
use sidecar_types::{Limit, Offset};

pub fn run(root: &str, sidecar_dir: &str, query: &str, limit: u32, offset: u32, json: bool) -> i32 {
    let limit = match Limit::new(limit) {
        Ok(l) => l,
        Err(e) => {
            if json {
                println!("{}", serde_json::json!({"error": format!("{e}")}));
            } else {
                eprintln!("sidecar search: {e}");
            }
            return 2;
        }
    };
    let offset = Offset::new(offset);

    let db_path = Path::new(root).join(sidecar_dir).join("index.sqlite");
    let repo = match SqliteRepository::open(&db_path) {
        Ok(r) => r,
        Err(e) => {
            if json {
                println!("{}", serde_json::json!({"error": format!("{e}")}));
            } else {
                eprintln!("sidecar search: {e}");
            }
            return 3;
        }
    };

    match repo.search_symbols(&SearchQuery {
        query: query.to_string(),
        limit,
        offset,
    }) {
        Ok(result) => {
            if json {
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            } else if result.results.is_empty() {
                eprintln!("No symbols found matching '{query}'");
            } else {
                for sym in &result.results {
                    eprintln!(
                        "{:<40} {:<12} {}",
                        sym.qualified_name,
                        format!("{:?}", sym.kind),
                        sym.uid
                    );
                }
                if result.truncated {
                    eprintln!("... (truncated, use --limit/--offset for more)");
                }
            }
            0
        }
        Err(e) => {
            if json {
                println!("{}", serde_json::json!({"error": format!("{e}")}));
            } else {
                eprintln!("sidecar search: {e}");
            }
            e.exit_code()
        }
    }
}
