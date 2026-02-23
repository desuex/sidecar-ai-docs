use std::path::Path;

use rusqlite::{params, Connection, Row};
use sidecar_types::{
    ContentHash, Fingerprint, Language, PathRel, Range, RefKind, SidecarError, SymbolKind, Uid,
    Visibility,
};

use sidecar_core::model::{DocRecord, FileRecord, Reference, Symbol};
use sidecar_core::query::{RefsQuery, RefsResult, SearchQuery, SearchResult};
use sidecar_core::Repository;

use crate::migrations;

/// SQLite-backed implementation of the Repository trait.
pub struct SqliteRepository {
    conn: Connection,
}

impl SqliteRepository {
    /// Open (or create) the index database at the given path.
    pub fn open(path: &Path) -> Result<Self, SidecarError> {
        let conn = Connection::open(path).map_err(|e| SidecarError::Index(e.to_string()))?;

        // WAL mode for better concurrent read performance
        conn.execute_batch("PRAGMA journal_mode=WAL;")
            .map_err(|e| SidecarError::Index(e.to_string()))?;

        migrations::migrate_to_latest(&conn)?;

        Ok(SqliteRepository { conn })
    }

    /// Open an in-memory database (for testing).
    pub fn open_in_memory() -> Result<Self, SidecarError> {
        let conn = Connection::open_in_memory().map_err(|e| SidecarError::Index(e.to_string()))?;
        migrations::migrate_to_latest(&conn)?;
        Ok(SqliteRepository { conn })
    }
}

fn row_to_symbol(row: &Row) -> Result<Symbol, rusqlite::Error> {
    let uid_str: String = row.get("uid")?;
    let file_uid_str: String = row.get("file_uid")?;
    let kind_str: String = row.get("kind")?;
    let visibility_str: String = row.get("visibility")?;
    let fingerprint_str: String = row.get("fingerprint")?;
    let range_start: u32 = row.get("range_start")?;
    let range_end: u32 = row.get("range_end")?;

    Ok(Symbol {
        uid: uid_str.parse().map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
        })?,
        file_uid: file_uid_str.parse().map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Text, Box::new(e))
        })?,
        kind: serde_json::from_str::<SymbolKind>(&format!("\"{kind_str}\""))
            .unwrap_or(SymbolKind::Variable),
        qualified_name: row.get("qualified_name")?,
        name: row.get("name")?,
        visibility: serde_json::from_str::<Visibility>(&format!("\"{visibility_str}\""))
            .unwrap_or(Visibility::Unknown),
        fingerprint: Fingerprint::from_hex(fingerprint_str),
        range: Range {
            start: range_start,
            end: range_end,
        },
    })
}

fn row_to_file(row: &Row) -> Result<FileRecord, rusqlite::Error> {
    let file_uid_str: String = row.get("file_uid")?;
    let path_str: String = row.get("path")?;
    let lang_str: String = row.get("language")?;
    let hash_str: String = row.get("content_hash")?;

    Ok(FileRecord {
        file_uid: file_uid_str.parse().map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
        })?,
        path: path_str.parse().map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Text, Box::new(e))
        })?,
        language: serde_json::from_str::<Language>(&format!("\"{lang_str}\""))
            .unwrap_or(Language::TypeScript),
        content_hash: ContentHash::from_hex(hash_str),
        last_indexed_at: row.get("last_indexed_at")?,
    })
}

impl Repository for SqliteRepository {
    fn upsert_file(&self, file: &FileRecord) -> Result<(), SidecarError> {
        self.conn
            .execute(
                "INSERT OR REPLACE INTO files (file_uid, path, language, content_hash, last_indexed_at) \
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    file.file_uid.as_str(),
                    file.path.as_str(),
                    file.language.code(),
                    file.content_hash.as_str(),
                    file.last_indexed_at,
                ],
            )
            .map_err(|e| SidecarError::Index(e.to_string()))?;
        Ok(())
    }

    fn upsert_symbols(&self, symbols: &[Symbol]) -> Result<(), SidecarError> {
        let tx = self
            .conn
            .unchecked_transaction()
            .map_err(|e| SidecarError::Index(e.to_string()))?;

        // Delete old symbols for the same file (if any symbols provided)
        if let Some(first) = symbols.first() {
            tx.execute(
                "DELETE FROM symbols WHERE file_uid = ?1",
                params![first.file_uid.as_str()],
            )
            .map_err(|e| SidecarError::Index(e.to_string()))?;
        }

        for sym in symbols {
            let kind_json = serde_json::to_value(sym.kind).unwrap();
            let vis_json = serde_json::to_value(sym.visibility).unwrap();
            tx.execute(
                "INSERT INTO symbols (uid, file_uid, kind, qualified_name, name, visibility, fingerprint, range_start, range_end) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    sym.uid.as_str(),
                    sym.file_uid.as_str(),
                    kind_json.as_str().unwrap_or("variable"),
                    sym.qualified_name,
                    sym.name,
                    vis_json.as_str().unwrap_or("unknown"),
                    sym.fingerprint.as_str(),
                    sym.range.start,
                    sym.range.end,
                ],
            )
            .map_err(|e| SidecarError::Index(e.to_string()))?;
        }

        tx.commit()
            .map_err(|e| SidecarError::Index(e.to_string()))?;
        Ok(())
    }

    fn upsert_refs(&self, refs: &[Reference]) -> Result<(), SidecarError> {
        let tx = self
            .conn
            .unchecked_transaction()
            .map_err(|e| SidecarError::Index(e.to_string()))?;

        // Delete old refs for the same file (if any refs provided)
        if let Some(first) = refs.first() {
            tx.execute(
                "DELETE FROM refs WHERE file_uid = ?1",
                params![first.file_uid.as_str()],
            )
            .map_err(|e| SidecarError::Index(e.to_string()))?;
        }

        for r in refs {
            let kind_json = serde_json::to_value(r.ref_kind).unwrap();
            tx.execute(
                "INSERT INTO refs (from_uid, to_uid, file_uid, range_start, range_end, ref_kind) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    r.from_uid.as_str(),
                    r.to_uid.as_str(),
                    r.file_uid.as_str(),
                    r.range.start,
                    r.range.end,
                    kind_json.as_str().unwrap_or("unknown"),
                ],
            )
            .map_err(|e| SidecarError::Index(e.to_string()))?;
        }

        tx.commit()
            .map_err(|e| SidecarError::Index(e.to_string()))?;
        Ok(())
    }

    fn get_file_by_path(&self, path: &PathRel) -> Result<Option<FileRecord>, SidecarError> {
        let mut stmt = self
            .conn
            .prepare("SELECT file_uid, path, language, content_hash, last_indexed_at FROM files WHERE path = ?1")
            .map_err(|e| SidecarError::Index(e.to_string()))?;

        let result = stmt.query_row(params![path.as_str()], row_to_file).ok();

        Ok(result)
    }

    fn search_symbols(&self, query: &SearchQuery) -> Result<SearchResult, SidecarError> {
        let pattern = format!("%{}%", query.query);
        let limit = query.limit.value();
        let offset = query.offset.value();

        let mut stmt = self
            .conn
            .prepare(
                "SELECT uid, file_uid, kind, qualified_name, name, visibility, fingerprint, range_start, range_end \
                 FROM symbols \
                 WHERE name LIKE ?1 OR qualified_name LIKE ?1 \
                 ORDER BY name ASC, uid ASC \
                 LIMIT ?2 OFFSET ?3",
            )
            .map_err(|e| SidecarError::Index(e.to_string()))?;

        let results: Vec<Symbol> = stmt
            .query_map(params![pattern, limit, offset], row_to_symbol)
            .map_err(|e| SidecarError::Index(e.to_string()))?
            .filter_map(|r| r.ok())
            .collect();

        let truncated = results.len() as u32 == limit;

        Ok(SearchResult { results, truncated })
    }

    fn get_symbol(&self, uid: &Uid) -> Result<Option<Symbol>, SidecarError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT uid, file_uid, kind, qualified_name, name, visibility, fingerprint, range_start, range_end \
                 FROM symbols WHERE uid = ?1",
            )
            .map_err(|e| SidecarError::Index(e.to_string()))?;

        let result = stmt.query_row(params![uid.as_str()], row_to_symbol).ok();

        Ok(result)
    }

    fn find_refs(&self, uid: &Uid, query: &RefsQuery) -> Result<RefsResult, SidecarError> {
        let limit = query.limit.value();
        let offset = query.offset.value();

        // Get total count
        let total: u32 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM refs WHERE to_uid = ?1",
                params![uid.as_str()],
                |row| row.get(0),
            )
            .map_err(|e| SidecarError::Index(e.to_string()))?;

        let mut stmt = self
            .conn
            .prepare(
                "SELECT from_uid, to_uid, file_uid, range_start, range_end, ref_kind \
                 FROM refs WHERE to_uid = ?1 \
                 ORDER BY file_uid ASC, range_start ASC \
                 LIMIT ?2 OFFSET ?3",
            )
            .map_err(|e| SidecarError::Index(e.to_string()))?;

        let results: Vec<Reference> = stmt
            .query_map(params![uid.as_str(), limit, offset], |row| {
                let from_uid_str: String = row.get("from_uid")?;
                let to_uid_str: String = row.get("to_uid")?;
                let file_uid_str: String = row.get("file_uid")?;
                let range_start: u32 = row.get("range_start")?;
                let range_end: u32 = row.get("range_end")?;
                let ref_kind_str: String = row.get("ref_kind")?;

                Ok((
                    from_uid_str,
                    to_uid_str,
                    file_uid_str,
                    range_start,
                    range_end,
                    ref_kind_str,
                ))
            })
            .map_err(|e| SidecarError::Index(e.to_string()))?
            .filter_map(|r| r.ok())
            .filter_map(
                |(from_uid_str, to_uid_str, file_uid_str, range_start, range_end, ref_kind_str)| {
                    let from_uid: Uid = from_uid_str.parse().ok()?;
                    let to_uid: Uid = to_uid_str.parse().ok()?;
                    let file_uid: Uid = file_uid_str.parse().ok()?;
                    let ref_kind: RefKind = serde_json::from_str(&format!("\"{ref_kind_str}\""))
                        .unwrap_or(RefKind::Unknown);
                    Some(Reference {
                        from_uid,
                        to_uid,
                        file_uid,
                        range: Range {
                            start: range_start,
                            end: range_end,
                        },
                        ref_kind,
                    })
                },
            )
            .collect();

        let truncated = results.len() as u32 == limit;

        Ok(RefsResult {
            total,
            results,
            truncated,
        })
    }

    fn get_doc(&self, uid: &Uid) -> Result<Option<DocRecord>, SidecarError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT doc_uid, target_uid, path, summary_cache, updated_at \
                 FROM docs WHERE target_uid = ?1",
            )
            .map_err(|e| SidecarError::Index(e.to_string()))?;

        let result = stmt
            .query_row(params![uid.as_str()], |row| {
                let doc_uid_str: String = row.get("doc_uid")?;
                let target_uid_str: String = row.get("target_uid")?;
                let path_str: String = row.get("path")?;
                let summary_cache: Option<String> = row.get("summary_cache")?;
                let updated_at: String = row.get("updated_at")?;

                Ok((
                    doc_uid_str,
                    target_uid_str,
                    path_str,
                    summary_cache,
                    updated_at,
                ))
            })
            .ok();

        match result {
            Some((doc_uid_str, target_uid_str, path_str, summary_cache, updated_at)) => {
                let doc_uid: Uid = doc_uid_str
                    .parse()
                    .map_err(|_| SidecarError::Index(format!("invalid doc_uid: {doc_uid_str}")))?;
                let target_uid: Uid = target_uid_str.parse().map_err(|_| {
                    SidecarError::Index(format!("invalid target_uid: {target_uid_str}"))
                })?;
                let path: PathRel = path_str
                    .parse()
                    .map_err(|_| SidecarError::Index(format!("invalid doc path: {path_str}")))?;

                Ok(Some(DocRecord {
                    doc_uid,
                    target_uid,
                    path,
                    summary_cache,
                    updated_at,
                }))
            }
            None => Ok(None),
        }
    }

    fn upsert_docs(&self, docs: &[DocRecord]) -> Result<(), SidecarError> {
        let tx = self
            .conn
            .unchecked_transaction()
            .map_err(|e| SidecarError::Index(e.to_string()))?;

        for doc in docs {
            tx.execute(
                "INSERT OR REPLACE INTO docs (doc_uid, target_uid, path, summary_cache, updated_at) \
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    doc.doc_uid.as_str(),
                    doc.target_uid.as_str(),
                    doc.path.as_str(),
                    doc.summary_cache,
                    doc.updated_at,
                ],
            )
            .map_err(|e| SidecarError::Index(e.to_string()))?;
        }

        tx.commit()
            .map_err(|e| SidecarError::Index(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_in_memory_succeeds() {
        let _repo = SqliteRepository::open_in_memory().unwrap();
    }

    #[test]
    fn upsert_and_search_symbols() {
        let repo = SqliteRepository::open_in_memory().unwrap();

        let file = FileRecord {
            file_uid: "file:src/test.ts".parse().unwrap(),
            path: "src/test.ts".parse().unwrap(),
            language: Language::TypeScript,
            content_hash: ContentHash::from_hex("aabbccdd".to_string()),
            last_indexed_at: "2026-01-01T00:00:00Z".to_string(),
        };
        repo.upsert_file(&file).unwrap();

        let symbols = vec![
            Symbol {
                uid: "sym:ts:src/test:Foo.bar:abcd1234".parse().unwrap(),
                file_uid: "file:src/test.ts".parse().unwrap(),
                kind: SymbolKind::Method,
                qualified_name: "Foo.bar".to_string(),
                name: "bar".to_string(),
                visibility: Visibility::Public,
                fingerprint: Fingerprint::from_hex("abcd1234abcd1234".to_string()),
                range: Range { start: 10, end: 50 },
            },
            Symbol {
                uid: "sym:ts:src/test:Foo:ef560078".parse().unwrap(),
                file_uid: "file:src/test.ts".parse().unwrap(),
                kind: SymbolKind::Class,
                qualified_name: "Foo".to_string(),
                name: "Foo".to_string(),
                visibility: Visibility::Public,
                fingerprint: Fingerprint::from_hex("ef560078ef560078".to_string()),
                range: Range { start: 0, end: 100 },
            },
        ];
        repo.upsert_symbols(&symbols).unwrap();

        // Search by name
        let result = repo
            .search_symbols(&SearchQuery {
                query: "bar".to_string(),
                limit: sidecar_types::Limit::default(),
                offset: sidecar_types::Offset::default(),
            })
            .unwrap();
        assert_eq!(result.results.len(), 1);
        assert_eq!(result.results[0].name, "bar");

        // Search by qualified name
        let result = repo
            .search_symbols(&SearchQuery {
                query: "Foo".to_string(),
                limit: sidecar_types::Limit::default(),
                offset: sidecar_types::Offset::default(),
            })
            .unwrap();
        assert_eq!(result.results.len(), 2);
        // Should be sorted: Foo before bar (by name ASC)
        assert_eq!(result.results[0].name, "Foo");
        assert_eq!(result.results[1].name, "bar");
    }

    #[test]
    fn get_symbol_by_uid() {
        let repo = SqliteRepository::open_in_memory().unwrap();

        let file = FileRecord {
            file_uid: "file:src/test.ts".parse().unwrap(),
            path: "src/test.ts".parse().unwrap(),
            language: Language::TypeScript,
            content_hash: ContentHash::from_hex("aabb".to_string()),
            last_indexed_at: "2026-01-01".to_string(),
        };
        repo.upsert_file(&file).unwrap();

        let sym = Symbol {
            uid: "sym:ts:src/test:MyFunc:abcd1234".parse().unwrap(),
            file_uid: "file:src/test.ts".parse().unwrap(),
            kind: SymbolKind::Function,
            qualified_name: "MyFunc".to_string(),
            name: "MyFunc".to_string(),
            visibility: Visibility::Public,
            fingerprint: Fingerprint::from_hex("abcd1234abcd1234".to_string()),
            range: Range { start: 0, end: 20 },
        };
        repo.upsert_symbols(&[sym]).unwrap();

        let found = repo
            .get_symbol(&"sym:ts:src/test:MyFunc:abcd1234".parse().unwrap())
            .unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "MyFunc");

        let missing = repo
            .get_symbol(&"sym:ts:src/test:NoExist:00001111".parse().unwrap())
            .unwrap();
        assert!(missing.is_none());
    }

    #[test]
    fn incremental_skip_unchanged() {
        let repo = SqliteRepository::open_in_memory().unwrap();

        let file = FileRecord {
            file_uid: "file:src/a.ts".parse().unwrap(),
            path: "src/a.ts".parse().unwrap(),
            language: Language::TypeScript,
            content_hash: ContentHash::from_hex("hash1".to_string()),
            last_indexed_at: "2026-01-01".to_string(),
        };
        repo.upsert_file(&file).unwrap();

        // Same hash → should find existing
        let found = repo.get_file_by_path(&"src/a.ts".parse().unwrap()).unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().content_hash.as_str(), "hash1");
    }
}
