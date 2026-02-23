use std::path::Path;

use sidecar_core::Repository;
use sidecar_storage::SqliteRepository;
use sidecar_types::Uid;

pub fn run(root: &str, sidecar_dir: &str, uid_str: &str, json: bool) -> i32 {
    let uid: Uid = match uid_str.parse() {
        Ok(u) => u,
        Err(e) => {
            if json {
                println!(
                    "{}",
                    serde_json::json!({"error": format!("invalid UID: {e}")})
                );
            } else {
                eprintln!("sidecar symbol: invalid UID: {e}");
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
                eprintln!("sidecar symbol: {e}");
            }
            return 3;
        }
    };

    match repo.get_symbol(&uid) {
        Ok(Some(sym)) => {
            if json {
                println!("{}", serde_json::to_string_pretty(&sym).unwrap());
            } else {
                eprintln!("{} ({:?}) {}", sym.qualified_name, sym.kind, sym.uid);
                eprintln!("  file: {}", sym.file_uid);
                eprintln!("  range: {}..{}", sym.range.start, sym.range.end);
                eprintln!("  visibility: {:?}", sym.visibility);
            }
            0
        }
        Ok(None) => {
            if json {
                println!(
                    "{}",
                    serde_json::json!({"error": "not found", "uid": uid_str})
                );
            } else {
                eprintln!("sidecar symbol: not found: {uid_str}");
            }
            1
        }
        Err(e) => {
            if json {
                println!("{}", serde_json::json!({"error": format!("{e}")}));
            } else {
                eprintln!("sidecar symbol: {e}");
            }
            e.exit_code()
        }
    }
}
