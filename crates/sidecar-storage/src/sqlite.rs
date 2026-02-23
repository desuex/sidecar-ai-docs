use std::path::Path;

use rusqlite::Connection;
use sidecar_types::{SidecarError, Uid};

use sidecar_core::model::{DocRecord, FileRecord, Reference, Symbol};
use sidecar_core::query::{RefsQuery, RefsResult, SearchQuery, SearchResult};
use sidecar_core::Repository;

use crate::migrations;

/// SQLite-backed implementation of the Repository trait.
pub struct SqliteRepository {
    // Will be used in M1 when query methods are implemented.
    #[allow(dead_code)]
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

impl Repository for SqliteRepository {
    fn upsert_file(&self, _file: &FileRecord) -> Result<(), SidecarError> {
        // TODO(M1): INSERT OR REPLACE into files table
        Ok(())
    }

    fn upsert_symbols(&self, _symbols: &[Symbol]) -> Result<(), SidecarError> {
        // TODO(M1): INSERT OR REPLACE into symbols table
        Ok(())
    }

    fn upsert_refs(&self, _refs: &[Reference]) -> Result<(), SidecarError> {
        // TODO(M2): INSERT into refs table
        Ok(())
    }

    fn search_symbols(&self, _query: &SearchQuery) -> Result<SearchResult, SidecarError> {
        // TODO(M1): SELECT from symbols with LIKE matching + ORDER BY
        Ok(SearchResult {
            results: Vec::new(),
            truncated: false,
        })
    }

    fn get_symbol(&self, _uid: &Uid) -> Result<Option<Symbol>, SidecarError> {
        // TODO(M1): SELECT from symbols WHERE uid = ?
        Ok(None)
    }

    fn find_refs(&self, _uid: &Uid, _query: &RefsQuery) -> Result<RefsResult, SidecarError> {
        // TODO(M2): SELECT from refs WHERE to_uid = ? ORDER BY ...
        Ok(RefsResult {
            total: 0,
            results: Vec::new(),
            truncated: false,
        })
    }

    fn get_doc(&self, _uid: &Uid) -> Result<Option<DocRecord>, SidecarError> {
        // TODO(M4): SELECT from docs WHERE target_uid = ?
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_in_memory_succeeds() {
        let _repo = SqliteRepository::open_in_memory().unwrap();
    }
}
