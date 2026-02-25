use std::path::{Path, PathBuf};
use std::process::Command;

pub fn run_mkdocs(root: &str, out: &str, index_db: Option<&str>) -> i32 {
    let root_path = Path::new(root);
    if !root_path.is_dir() {
        eprintln!("sidecar export mkdocs: root is not a directory: {root}");
        return 5;
    }
    let root_abs = match root_path.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("sidecar export mkdocs: cannot resolve root path '{root}': {e}");
            return 5;
        }
    };

    let script_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../scripts/docs/export-sidecar-to-mkdocs.sh");
    if !script_path.is_file() {
        eprintln!(
            "sidecar export mkdocs: exporter script not found: {}",
            script_path.display()
        );
        return 5;
    }

    let out_path = if Path::new(out).is_absolute() {
        PathBuf::from(out)
    } else {
        root_abs.join(out)
    };
    let index_db_path = match index_db {
        Some(path) if Path::new(path).is_absolute() => PathBuf::from(path),
        Some(path) => root_abs.join(path),
        None => root_abs.join(".sidecar").join("index.sqlite"),
    };

    let status = Command::new("bash")
        .arg(&script_path)
        .current_dir(&root_abs)
        .env("SIDECAR_EXPORT_OUT_DIR", &out_path)
        .env("SIDECAR_INDEX_DB_PATH", &index_db_path)
        .status();

    match status {
        Ok(s) => {
            if s.success() {
                0
            } else {
                s.code().unwrap_or(5)
            }
        }
        Err(e) => {
            eprintln!("sidecar export mkdocs: failed to run exporter: {e}");
            5
        }
    }
}
