/// Schema version. Bump when DDL changes.
pub const SCHEMA_VERSION: u32 = 1;

/// UID format version. Bump when UID generation algorithm changes.
pub const UID_FORMAT_VERSION: u32 = 1;

pub const CREATE_META: &str = "
CREATE TABLE IF NOT EXISTS meta (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
";

pub const CREATE_FILES: &str = "
CREATE TABLE IF NOT EXISTS files (
    file_uid        TEXT PRIMARY KEY,
    path            TEXT NOT NULL UNIQUE,
    language        TEXT NOT NULL,
    content_hash    TEXT NOT NULL,
    last_indexed_at TEXT NOT NULL
);
";

pub const CREATE_SYMBOLS: &str = "
CREATE TABLE IF NOT EXISTS symbols (
    uid             TEXT PRIMARY KEY,
    file_uid        TEXT NOT NULL REFERENCES files(file_uid),
    kind            TEXT NOT NULL,
    qualified_name  TEXT NOT NULL,
    name            TEXT NOT NULL,
    visibility      TEXT NOT NULL,
    fingerprint     TEXT NOT NULL,
    range_start     INTEGER NOT NULL,
    range_end       INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_symbols_name ON symbols(name);
CREATE INDEX IF NOT EXISTS idx_symbols_qualified_name ON symbols(qualified_name);
CREATE INDEX IF NOT EXISTS idx_symbols_file_uid ON symbols(file_uid);
";

pub const CREATE_REFS: &str = "
CREATE TABLE IF NOT EXISTS refs (
    from_uid    TEXT NOT NULL,
    to_uid      TEXT NOT NULL,
    file_uid    TEXT NOT NULL REFERENCES files(file_uid),
    range_start INTEGER NOT NULL,
    range_end   INTEGER NOT NULL,
    ref_kind    TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_refs_to_uid ON refs(to_uid);
CREATE INDEX IF NOT EXISTS idx_refs_from_uid ON refs(from_uid);
";

pub const CREATE_DOCS: &str = "
CREATE TABLE IF NOT EXISTS docs (
    doc_uid       TEXT PRIMARY KEY,
    target_uid    TEXT NOT NULL,
    path          TEXT NOT NULL,
    summary_cache TEXT,
    updated_at    TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_docs_target_uid ON docs(target_uid);
";
