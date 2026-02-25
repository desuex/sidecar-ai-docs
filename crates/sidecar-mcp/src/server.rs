use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

use sidecar_core::Repository;

use crate::protocol::{JsonRpcRequest, JsonRpcResponse};
use crate::tools;

/// MCP server that reads JSON-RPC from stdin and writes to stdout.
pub struct McpServer<R: Repository> {
    repo: R,
    root: PathBuf,
}

impl<R: Repository> McpServer<R> {
    pub fn new(repo: R, root: &Path) -> Self {
        McpServer {
            repo,
            root: root.to_path_buf(),
        }
    }

    /// Run the stdio read loop. Blocks until EOF.
    pub fn run(&self) -> Result<(), io::Error> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        self.run_with_io(stdin.lock(), &mut stdout)
    }

    fn run_with_io<B: BufRead, W: Write>(
        &self,
        mut reader: B,
        writer: &mut W,
    ) -> Result<(), io::Error> {
        let mut line = String::new();

        loop {
            line.clear();
            if reader.read_line(&mut line)? == 0 {
                break;
            }
            if line.trim().is_empty() {
                continue;
            }

            let response = match serde_json::from_str::<JsonRpcRequest>(&line) {
                Ok(req) => tools::dispatch(&self.repo, &req, &self.root),
                Err(e) => JsonRpcResponse::error(
                    serde_json::Value::Null,
                    -32700,
                    format!("Parse error: {e}"),
                ),
            };

            let out = serde_json::to_string(&response).unwrap();
            writeln!(writer, "{out}")?;
            writer.flush()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use sidecar_core::model::{DocRecord, FileRecord, Reference, Symbol};
    use sidecar_core::query::{RefsQuery, RefsResult, SearchQuery, SearchResult};
    use sidecar_core::Repository;
    use sidecar_types::{PathRel, SidecarError, Uid};

    use super::McpServer;

    struct MockRepo;

    impl Repository for MockRepo {
        fn upsert_file(&self, _file: &FileRecord) -> Result<(), SidecarError> {
            Ok(())
        }

        fn upsert_symbols(&self, _symbols: &[Symbol]) -> Result<(), SidecarError> {
            Ok(())
        }

        fn upsert_refs(&self, _refs: &[Reference]) -> Result<(), SidecarError> {
            Ok(())
        }

        fn get_file_by_path(&self, _path: &PathRel) -> Result<Option<FileRecord>, SidecarError> {
            Ok(None)
        }

        fn search_symbols(&self, _query: &SearchQuery) -> Result<SearchResult, SidecarError> {
            Ok(SearchResult {
                results: Vec::new(),
                truncated: false,
            })
        }

        fn get_symbol(&self, _uid: &Uid) -> Result<Option<Symbol>, SidecarError> {
            Ok(None)
        }

        fn find_refs(&self, _uid: &Uid, _query: &RefsQuery) -> Result<RefsResult, SidecarError> {
            Ok(RefsResult {
                total: 0,
                results: Vec::new(),
                truncated: false,
            })
        }

        fn get_doc(&self, _uid: &Uid) -> Result<Option<DocRecord>, SidecarError> {
            Ok(None)
        }

        fn upsert_docs(&self, _docs: &[DocRecord]) -> Result<(), SidecarError> {
            Ok(())
        }
    }

    #[test]
    fn handles_parse_error_line() {
        let server = McpServer::new(MockRepo, std::path::Path::new("."));
        let input = Cursor::new("{oops}\n");
        let mut output = Vec::new();

        server.run_with_io(input, &mut output).unwrap();
        let text = String::from_utf8(output).unwrap();
        assert!(text.contains("\"code\":-32700"));
        assert!(text.contains("Parse error"));
    }

    #[test]
    fn handles_valid_request_and_skips_blank_lines() {
        let server = McpServer::new(MockRepo, std::path::Path::new("."));
        let input = Cursor::new(
            "\n{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"search_symbols\",\"params\":{\"query\":\"Cart\"}}\n",
        );
        let mut output = Vec::new();

        server.run_with_io(input, &mut output).unwrap();
        let text = String::from_utf8(output).unwrap();
        assert!(text.contains("\"jsonrpc\":\"2.0\""));
        assert!(text.contains("\"results\":[]"));
    }
}
