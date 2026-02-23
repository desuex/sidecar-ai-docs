use sidecar_types::{SidecarError, Uid};

use crate::model::{DocRecord, FileRecord, Reference, Symbol};
use crate::query::{RefsQuery, RefsResult, SearchQuery, SearchResult};

/// Storage abstraction. Core depends on this trait; storage crate provides the impl.
pub trait Repository {
    fn upsert_file(&self, file: &FileRecord) -> Result<(), SidecarError>;
    fn upsert_symbols(&self, symbols: &[Symbol]) -> Result<(), SidecarError>;
    fn upsert_refs(&self, refs: &[Reference]) -> Result<(), SidecarError>;

    fn search_symbols(&self, query: &SearchQuery) -> Result<SearchResult, SidecarError>;
    fn get_symbol(&self, uid: &Uid) -> Result<Option<Symbol>, SidecarError>;
    fn find_refs(&self, uid: &Uid, query: &RefsQuery) -> Result<RefsResult, SidecarError>;

    fn get_doc(&self, uid: &Uid) -> Result<Option<DocRecord>, SidecarError>;
}
