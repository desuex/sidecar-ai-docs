use std::path::Path;

use sidecar_core::doc_parser;
use sidecar_core::Repository;
use sidecar_storage::SqliteRepository;

pub fn run(root: &str, sidecar_dir: &str, uid: &str, mode: &str, json: bool) -> i32 {
    let uid = match uid.parse::<sidecar_types::Uid>() {
        Ok(u) => u,
        Err(e) => {
            if json {
                println!("{}", serde_json::json!({"error": format!("{e}")}));
            } else {
                eprintln!("sidecar doc: invalid UID: {e}");
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
                eprintln!("sidecar doc: {e}");
            }
            return 3;
        }
    };

    match repo.get_doc(&uid) {
        Ok(Some(doc)) => {
            // Read the actual doc file from disk
            let doc_file_path = Path::new(root).join(doc.path.as_str());
            let content = match std::fs::read_to_string(&doc_file_path) {
                Ok(c) => c,
                Err(e) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::json!({"error": format!("cannot read doc file: {e}")})
                        );
                    } else {
                        eprintln!("sidecar doc: cannot read doc file: {e}");
                    }
                    return 5;
                }
            };

            let (front_matter, body) = match doc_parser::parse_sidecar_doc(&content) {
                Ok(r) => r,
                Err(e) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::json!({"error": format!("cannot parse doc: {e}")})
                        );
                    } else {
                        eprintln!("sidecar doc: cannot parse doc: {e}");
                    }
                    return 5;
                }
            };

            let summary = doc_parser::extract_summary(&body);

            if json {
                let output = match mode {
                    "full" => serde_json::json!({
                        "exists": true,
                        "doc_uid": doc.doc_uid.as_str(),
                        "target_uid": doc.target_uid.as_str(),
                        "title": front_matter.title,
                        "summary": summary,
                        "content": body,
                    }),
                    _ => serde_json::json!({
                        "exists": true,
                        "doc_uid": doc.doc_uid.as_str(),
                        "target_uid": doc.target_uid.as_str(),
                        "title": front_matter.title,
                        "summary": summary,
                    }),
                };
                println!("{}", serde_json::to_string_pretty(&output).unwrap());
            } else {
                eprintln!("# {}", front_matter.title);
                eprintln!();
                match mode {
                    "full" => eprintln!("{body}"),
                    _ => {
                        if let Some(ref s) = summary {
                            eprintln!("{s}");
                        } else {
                            eprintln!("(no summary available)");
                        }
                    }
                }
            }
            0
        }
        Ok(None) => {
            if json {
                println!("{}", serde_json::json!({"exists": false}));
            } else {
                eprintln!("No documentation found for {}", uid.as_str());
            }
            1
        }
        Err(e) => {
            if json {
                println!("{}", serde_json::json!({"error": format!("{e}")}));
            } else {
                eprintln!("sidecar doc: {e}");
            }
            e.exit_code()
        }
    }
}
