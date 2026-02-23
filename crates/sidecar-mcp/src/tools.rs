use sidecar_core::Repository;

use crate::protocol::{JsonRpcRequest, JsonRpcResponse};

/// Dispatch an MCP request to the appropriate tool handler.
pub fn dispatch<R: Repository>(repo: &R, req: &JsonRpcRequest) -> JsonRpcResponse {
    match req.method.as_str() {
        "search_symbols" => handle_search_symbols(repo, req),
        "get_symbol" => handle_get_symbol(repo, req),
        "find_references" => handle_find_references(repo, req),
        "get_documentation" => handle_get_documentation(repo, req),
        _ => JsonRpcResponse::error(
            req.id.clone(),
            -32601,
            format!("Method not found: {}", req.method),
        ),
    }
}

fn handle_search_symbols<R: Repository>(_repo: &R, req: &JsonRpcRequest) -> JsonRpcResponse {
    // TODO(M3): Parse params, call repo.search_symbols, return results
    JsonRpcResponse::error(req.id.clone(), -32000, "Not implemented".to_owned())
}

fn handle_get_symbol<R: Repository>(_repo: &R, req: &JsonRpcRequest) -> JsonRpcResponse {
    // TODO(M3): Parse params, call repo.get_symbol, return result
    JsonRpcResponse::error(req.id.clone(), -32000, "Not implemented".to_owned())
}

fn handle_find_references<R: Repository>(_repo: &R, req: &JsonRpcRequest) -> JsonRpcResponse {
    // TODO(M3): Parse params, call repo.find_refs, return results
    JsonRpcResponse::error(req.id.clone(), -32000, "Not implemented".to_owned())
}

fn handle_get_documentation<R: Repository>(_repo: &R, req: &JsonRpcRequest) -> JsonRpcResponse {
    // TODO(M4): Parse params, call repo.get_doc, return result
    JsonRpcResponse::error(req.id.clone(), -32000, "Not implemented".to_owned())
}
