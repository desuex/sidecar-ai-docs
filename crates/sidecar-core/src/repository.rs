use sidecar_types::{PathRel, SidecarError, Uid};

use crate::model::{DocRecord, FileRecord, Reference, Symbol};
use crate::query::{RefsQuery, RefsResult, SearchQuery, SearchResult};

/// Storage abstraction. Core depends on this trait; storage crate provides the impl.
pub trait Repository {
    fn upsert_file(&self, file: &FileRecord) -> Result<(), SidecarError>;
    fn upsert_symbols(&self, symbols: &[Symbol]) -> Result<(), SidecarError>;
    fn upsert_refs(&self, refs: &[Reference]) -> Result<(), SidecarError>;

    /// Look up a file by its repo-relative path (for incremental indexing).
    fn get_file_by_path(&self, path: &PathRel) -> Result<Option<FileRecord>, SidecarError>;

    fn search_symbols(&self, query: &SearchQuery) -> Result<SearchResult, SidecarError>;
    fn get_symbol(&self, uid: &Uid) -> Result<Option<Symbol>, SidecarError>;
    fn find_refs(&self, uid: &Uid, query: &RefsQuery) -> Result<RefsResult, SidecarError>;

    fn get_doc(&self, uid: &Uid) -> Result<Option<DocRecord>, SidecarError>;
}
