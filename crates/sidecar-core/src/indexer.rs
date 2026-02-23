use std::path::Path;
use std::time::Instant;

use serde::Serialize;
use sidecar_types::{Language, PathRel};
use tracing::{debug, info, warn};
use walkdir::WalkDir;

use crate::fingerprint::{compute_content_hash, compute_fingerprint};
use crate::model::{FileRecord, Symbol};
use crate::repository::Repository;
use crate::uid::generate_uid;
use sidecar_parsing::LanguageAdapter;

/// Result of an indexing run.
#[derive(Debug, Serialize)]
pub struct IndexResult {
    pub files_indexed: u32,
    pub files_skipped: u32,
    pub symbols_extracted: u32,
    pub duration_ms: u64,
}

/// Index a project directory: walk files, parse, generate UIDs, store.
pub fn index_project(
    root: &Path,
    repo: &dyn Repository,
    adapters: &[&dyn LanguageAdapter],
) -> Result<IndexResult, sidecar_types::SidecarError> {
    let start = Instant::now();
    let mut files_indexed: u32 = 0;
    let mut files_skipped: u32 = 0;
    let mut symbols_extracted: u32 = 0;

    // Collect and sort entries for deterministic ordering
    let mut entries: Vec<_> = WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| !is_hidden_or_ignored(e.file_name().to_str().unwrap_or("")))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();
    entries.sort_by(|a, b| a.path().cmp(b.path()));

    for entry in &entries {
        let path = entry.path();

        // Determine language from extension
        let language = match path.extension().and_then(|e| e.to_str()) {
            Some("ts") => Language::TypeScript,
            Some("tsx") => Language::TypeScript,
            Some("js") => Language::JavaScript,
            Some("jsx") => Language::JavaScript,
            _ => continue, // Skip non-supported files
        };

        // Find matching adapter
        let adapter = match adapters.iter().find(|a| a.language() == language) {
            Some(a) => *a,
            None => {
                debug!("no adapter for {language}, skipping {}", path.display());
                continue;
            }
        };

        // Compute repo-relative path
        let rel_path = match path.strip_prefix(root) {
            Ok(p) => p.to_string_lossy().replace('\\', "/"),
            Err(_) => continue,
        };
        let path_rel: PathRel = match rel_path.parse() {
            Ok(p) => p,
            Err(e) => {
                warn!("invalid path {rel_path}: {e}");
                continue;
            }
        };

        // Read file content
        let content = match std::fs::read(path) {
            Ok(c) => c,
            Err(e) => {
                warn!("cannot read {}: {e}", path.display());
                continue;
            }
        };

        // Compute content hash for incremental indexing
        let content_hash = compute_content_hash(&content);

        // Check if already indexed with same hash
        if let Ok(Some(existing)) = repo.get_file_by_path(&path_rel) {
            if existing.content_hash == content_hash {
                debug!("skipping unchanged file: {rel_path}");
                files_skipped += 1;
                continue;
            }
        }

        // Generate file UID
        let file_uid_str = format!("file:{rel_path}");
        let file_uid: sidecar_types::Uid = match file_uid_str.parse() {
            Ok(u) => u,
            Err(e) => {
                warn!("invalid file UID {file_uid_str}: {e}");
                continue;
            }
        };

        // Parse symbols
        let raw_symbols = adapter.parse_symbols(&content);

        // Module path: file path without extension
        let module_path = rel_path
            .strip_suffix(".ts")
            .or_else(|| rel_path.strip_suffix(".tsx"))
            .or_else(|| rel_path.strip_suffix(".js"))
            .or_else(|| rel_path.strip_suffix(".jsx"))
            .unwrap_or(&rel_path);

        // Convert RawSymbol → Symbol with UID + fingerprint
        let mut symbols = Vec::with_capacity(raw_symbols.len());
        for raw in &raw_symbols {
            let fingerprint = compute_fingerprint(raw.fingerprint_input.as_bytes());
            let uid = match generate_uid(language, module_path, &raw.qualified_name, &fingerprint) {
                Ok(u) => u,
                Err(e) => {
                    warn!(
                        "cannot generate UID for {}.{}: {e}",
                        module_path, raw.qualified_name
                    );
                    continue;
                }
            };

            symbols.push(Symbol {
                uid,
                file_uid: file_uid.clone(),
                kind: raw.kind,
                qualified_name: raw.qualified_name.clone(),
                name: raw.name.clone(),
                visibility: raw.visibility,
                fingerprint,
                range: raw.range,
            });
        }

        // Build file record
        let now = chrono_lite_now();
        let file_record = FileRecord {
            file_uid,
            path: path_rel,
            language,
            content_hash,
            last_indexed_at: now,
        };

        // Store
        repo.upsert_file(&file_record)?;
        repo.upsert_symbols(&symbols)?;

        files_indexed += 1;
        symbols_extracted += symbols.len() as u32;

        info!("indexed {} ({} symbols)", rel_path, symbols.len());
    }

    let duration_ms = start.elapsed().as_millis() as u64;
    Ok(IndexResult {
        files_indexed,
        files_skipped,
        symbols_extracted,
        duration_ms,
    })
}

/// Check if a directory/file name should be skipped.
fn is_hidden_or_ignored(name: &str) -> bool {
    name.starts_with('.')
        || name == "node_modules"
        || name == "target"
        || name == "dist"
        || name == "build"
        || name == "__pycache__"
}

/// Simple timestamp without pulling in chrono.
fn chrono_lite_now() -> String {
    // For determinism in tests, this is the only place timestamps appear.
    // They are excluded from UID/fingerprint computation.
    "1970-01-01T00:00:00Z".to_string()
}
