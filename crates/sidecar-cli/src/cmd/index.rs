use std::path::Path;

use sidecar_core::indexer;
use sidecar_parsing::{RustAdapter, TypeScriptAdapter};
use sidecar_storage::SqliteRepository;

pub fn run(root: &str, sidecar_dir: &str, json: bool) -> i32 {
    let root_path = Path::new(root);
    let db_dir = root_path.join(sidecar_dir);

    // Create sidecar directory if needed
    if let Err(e) = std::fs::create_dir_all(&db_dir) {
        let msg = format!("cannot create sidecar directory: {e}");
        if json {
            println!("{}", serde_json::json!({"error": msg}));
        } else {
            eprintln!("sidecar index: {msg}");
        }
        return 5;
    }

    let db_path = db_dir.join("index.sqlite");
    let repo = match SqliteRepository::open(&db_path) {
        Ok(r) => r,
        Err(e) => {
            let msg = format!("cannot open database: {e}");
            if json {
                println!("{}", serde_json::json!({"error": msg}));
            } else {
                eprintln!("sidecar index: {msg}");
            }
            return 3;
        }
    };

    let ts_adapter = TypeScriptAdapter::new();
    let rs_adapter = RustAdapter::new();
    let adapters: Vec<&dyn sidecar_parsing::LanguageAdapter> = vec![&ts_adapter, &rs_adapter];

    match indexer::index_project(root_path, &repo, &adapters) {
        Ok(result) => {
            // Also index sidecar doc files
            let doc_result = indexer::index_docs(root_path, "docs-sidecar", &repo);
            let docs_indexed = doc_result.as_ref().map(|r| r.docs_indexed).unwrap_or(0);

            if json {
                let mut output = serde_json::to_value(&result).unwrap();
                output["docs_indexed"] = serde_json::Value::from(docs_indexed);
                println!("{}", serde_json::to_string_pretty(&output).unwrap());
            } else {
                eprintln!(
                    "Indexed {} files ({} skipped), {} symbols, {} docs in {}ms",
                    result.files_indexed,
                    result.files_skipped,
                    result.symbols_extracted,
                    docs_indexed,
                    result.duration_ms,
                );
            }
            0
        }
        Err(e) => {
            let msg = format!("{e}");
            if json {
                println!("{}", serde_json::json!({"error": msg}));
            } else {
                eprintln!("sidecar index: {msg}");
            }
            e.exit_code()
        }
    }
}
