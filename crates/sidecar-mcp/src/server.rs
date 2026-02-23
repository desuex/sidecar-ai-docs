use std::io::{self, BufRead, Write};

use sidecar_core::Repository;

use crate::protocol::{JsonRpcRequest, JsonRpcResponse};
use crate::tools;

/// MCP server that reads JSON-RPC from stdin and writes to stdout.
pub struct McpServer<R: Repository> {
    repo: R,
}

impl<R: Repository> McpServer<R> {
    pub fn new(repo: R) -> Self {
        McpServer { repo }
    }

    /// Run the stdio read loop. Blocks until EOF.
    pub fn run(&self) -> Result<(), io::Error> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        for line in stdin.lock().lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            let response = match serde_json::from_str::<JsonRpcRequest>(&line) {
                Ok(req) => tools::dispatch(&self.repo, &req),
                Err(e) => JsonRpcResponse::error(
                    serde_json::Value::Null,
                    -32700,
                    format!("Parse error: {e}"),
                ),
            };

            let out = serde_json::to_string(&response).unwrap();
            writeln!(stdout, "{out}")?;
            stdout.flush()?;
        }

        Ok(())
    }
}
