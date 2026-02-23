use std::path::Path;

use sidecar_core::Repository;
use sidecar_storage::SqliteRepository;
use sidecar_types::{Limit, Offset};

pub fn run(root: &str, sidecar_dir: &str, uid: &str, limit: u32, offset: u32, json: bool) -> i32 {
    let limit = match Limit::new(limit) {
        Ok(l) => l,
        Err(e) => {
            if json {
                println!("{}", serde_json::json!({"error": format!("{e}")}));
            } else {
                eprintln!("sidecar refs: {e}");
            }
            return 2;
        }
    };
    let offset = Offset::new(offset);

    let uid = match uid.parse::<sidecar_types::Uid>() {
        Ok(u) => u,
        Err(e) => {
            if json {
                println!("{}", serde_json::json!({"error": format!("{e}")}));
            } else {
                eprintln!("sidecar refs: invalid UID: {e}");
            }
            return 2;
        }
    };

    let db_path = Path::new(root).join(sidecar_dir).join("index.sqlite");
    let repo = match SqliteRepository::open(&db_path) {
        Ok(r) => r,
        Err(e) => {
            if json {
                println!("{}", serde_json::json!({"error": format!("{e}")}));
            } else {
                eprintln!("sidecar refs: {e}");
            }
            return 1;
        }
    };

    let query = sidecar_core::query::RefsQuery { limit, offset };

    match repo.find_refs(&uid, &query) {
        Ok(result) => {
            if json {
                println!("{}", serde_json::to_string_pretty(&result).unwrap());
            } else if result.results.is_empty() {
                eprintln!("No references found for {}", uid.as_str());
            } else {
                eprintln!("References to {} ({} total):\n", uid.as_str(), result.total);
                for r in &result.results {
                    eprintln!(
                        "  {:?}  from {}  in {}  [{}-{}]",
                        r.ref_kind,
                        r.from_uid.as_str(),
                        r.file_uid.as_str(),
                        r.range.start,
                        r.range.end,
                    );
                }
                if result.truncated {
                    eprintln!("\n  (truncated, use --offset to paginate)");
                }
            }
            0
        }
        Err(e) => {
            if json {
                println!("{}", serde_json::json!({"error": format!("{e}")}));
            } else {
                eprintln!("sidecar refs: {e}");
            }
            e.exit_code()
        }
    }
}
