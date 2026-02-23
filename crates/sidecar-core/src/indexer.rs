use std::path::Path;
use std::time::Instant;

use serde::Serialize;
use sidecar_types::{Language, PathRel};
use tracing::{debug, info, warn};
use walkdir::WalkDir;

use crate::doc_parser;
use crate::fingerprint::{compute_content_hash, compute_fingerprint};
use crate::model::{DocRecord, FileRecord, Reference, Symbol};
use crate::repository::Repository;
use crate::uid::generate_uid;
use sidecar_parsing::LanguageAdapter;

/// Result of an indexing run.
#[derive(Debug, Serialize)]
pub struct IndexResult {
    pub files_indexed: u32,
    pub files_skipped: u32,
    pub symbols_extracted: u32,
    pub refs_extracted: u32,
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
            Some("rs") => Language::Rust,
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
            .or_else(|| rel_path.strip_suffix(".rs"))
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

    // === Pass 2: Extract and resolve references ===
    let mut refs_extracted: u32 = 0;

    for entry in &entries {
        let path = entry.path();

        let language = match path.extension().and_then(|e| e.to_str()) {
            Some("ts") | Some("tsx") => Language::TypeScript,
            Some("js") | Some("jsx") => Language::JavaScript,
            Some("rs") => Language::Rust,
            _ => continue,
        };

        let adapter = match adapters.iter().find(|a| a.language() == language) {
            Some(a) => *a,
            None => continue,
        };

        let rel_path = match path.strip_prefix(root) {
            Ok(p) => p.to_string_lossy().replace('\\', "/"),
            Err(_) => continue,
        };

        let file_uid_str = format!("file:{rel_path}");
        let file_uid: sidecar_types::Uid = match file_uid_str.parse() {
            Ok(u) => u,
            Err(_) => continue,
        };

        let content = match std::fs::read(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let raw_refs = adapter.parse_refs(&content);
        if raw_refs.is_empty() {
            continue;
        }

        let module_path = rel_path
            .strip_suffix(".ts")
            .or_else(|| rel_path.strip_suffix(".tsx"))
            .or_else(|| rel_path.strip_suffix(".js"))
            .or_else(|| rel_path.strip_suffix(".jsx"))
            .or_else(|| rel_path.strip_suffix(".rs"))
            .unwrap_or(&rel_path);

        let mut resolved_refs = Vec::new();

        for raw_ref in &raw_refs {
            // Resolve from_qualified_name → from_uid (search symbols in this file's scope)
            let from_uid = if raw_ref.from_qualified_name == "<file>" {
                file_uid.clone()
            } else {
                // Search for a symbol with this qualified name
                let search = crate::query::SearchQuery {
                    query: raw_ref.from_qualified_name.clone(),
                    limit: sidecar_types::Limit::default(),
                    offset: sidecar_types::Offset::default(),
                };
                match repo.search_symbols(&search) {
                    Ok(result) => {
                        match result
                            .results
                            .iter()
                            .find(|s| s.qualified_name == raw_ref.from_qualified_name)
                        {
                            Some(s) => s.uid.clone(),
                            None => continue, // Can't resolve from
                        }
                    }
                    Err(_) => continue,
                }
            };

            // Resolve to_name → to_uid
            let search = crate::query::SearchQuery {
                query: raw_ref.to_name.clone(),
                limit: sidecar_types::Limit::default(),
                offset: sidecar_types::Offset::default(),
            };
            let to_uid = match repo.search_symbols(&search) {
                Ok(result) => {
                    // Prefer exact name match
                    match result.results.iter().find(|s| s.name == raw_ref.to_name) {
                        Some(s) => s.uid.clone(),
                        None => continue, // Can't resolve to
                    }
                }
                Err(_) => continue,
            };

            resolved_refs.push(Reference {
                from_uid,
                to_uid,
                file_uid: file_uid.clone(),
                range: raw_ref.range,
                ref_kind: raw_ref.ref_kind,
            });
        }

        if !resolved_refs.is_empty() {
            let count = resolved_refs.len();
            repo.upsert_refs(&resolved_refs)?;
            refs_extracted += count as u32;
            debug!(
                "resolved {count}/{} refs in {}",
                raw_refs.len(),
                module_path
            );
        }
    }

    let duration_ms = start.elapsed().as_millis() as u64;
    Ok(IndexResult {
        files_indexed,
        files_skipped,
        symbols_extracted,
        refs_extracted,
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

/// Result of a doc indexing run.
#[derive(Debug, Serialize)]
pub struct DocIndexResult {
    pub docs_indexed: u32,
}

/// Scan a docs directory for sidecar doc files, parse YAML front matter,
/// and populate the docs table.
pub fn index_docs(
    root: &Path,
    docs_dir: &str,
    repo: &dyn Repository,
) -> Result<DocIndexResult, sidecar_types::SidecarError> {
    let docs_path = root.join(docs_dir);
    if !docs_path.exists() {
        debug!("docs dir not found: {}", docs_path.display());
        return Ok(DocIndexResult { docs_indexed: 0 });
    }

    let mut entries: Vec<_> = WalkDir::new(&docs_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
                && e.path().extension().and_then(|ext| ext.to_str()) == Some("md")
        })
        .collect();
    entries.sort_by(|a, b| a.path().cmp(b.path()));

    let mut docs = Vec::new();

    for entry in &entries {
        let content = match std::fs::read_to_string(entry.path()) {
            Ok(c) => c,
            Err(e) => {
                warn!("cannot read doc {}: {e}", entry.path().display());
                continue;
            }
        };

        let (front_matter, body) = match doc_parser::parse_sidecar_doc(&content) {
            Ok(r) => r,
            Err(e) => {
                debug!("skipping non-sidecar doc {}: {e}", entry.path().display());
                continue;
            }
        };

        let rel_path = match entry.path().strip_prefix(root) {
            Ok(p) => p.to_string_lossy().replace('\\', "/"),
            Err(_) => continue,
        };
        let path_rel: sidecar_types::PathRel = match rel_path.parse() {
            Ok(p) => p,
            Err(e) => {
                warn!("invalid doc path {rel_path}: {e}");
                continue;
            }
        };

        let doc_uid: sidecar_types::Uid = match front_matter.doc_uid.parse() {
            Ok(u) => u,
            Err(e) => {
                warn!("invalid doc_uid '{}': {e}", front_matter.doc_uid);
                continue;
            }
        };

        let summary = doc_parser::extract_summary(&body);
        let updated_at = front_matter
            .updated_at
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".to_string());

        for anchor in &front_matter.anchors {
            if anchor.anchor_type != "symbol" {
                continue;
            }
            let symbol_uid_str = match &anchor.symbol_uid {
                Some(s) => s,
                None => continue,
            };
            let target_uid: sidecar_types::Uid = match symbol_uid_str.parse() {
                Ok(u) => u,
                Err(e) => {
                    warn!("invalid anchor symbol_uid '{symbol_uid_str}': {e}");
                    continue;
                }
            };

            docs.push(DocRecord {
                doc_uid: doc_uid.clone(),
                target_uid,
                path: path_rel.clone(),
                summary_cache: summary.clone(),
                updated_at: updated_at.clone(),
            });
        }
    }

    let count = docs.len() as u32;
    repo.upsert_docs(&docs)?;

    info!("indexed {count} doc entries from {docs_dir}");
    Ok(DocIndexResult {
        docs_indexed: count,
    })
}

/// Simple timestamp without pulling in chrono.
fn chrono_lite_now() -> String {
    // For determinism in tests, this is the only place timestamps appear.
    // They are excluded from UID/fingerprint computation.
    "1970-01-01T00:00:00Z".to_string()
}
