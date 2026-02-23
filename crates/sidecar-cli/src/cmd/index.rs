use std::path::Path;

use sidecar_core::indexer;
use sidecar_parsing::TypeScriptAdapter;
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
    let adapters: Vec<&dyn sidecar_parsing::LanguageAdapter> = vec![&ts_adapter];

    match indexer::index_project(root_path, &repo, &adapters) {
        Ok(result) => {
            if json {
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            } else {
                eprintln!(
                    "Indexed {} files ({} skipped), {} symbols in {}ms",
                    result.files_indexed,
                    result.files_skipped,
                    result.symbols_extracted,
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
