use std::path::Path;

use serde_json::Value;
use sidecar_core::model::Symbol;
use sidecar_core::query::{RefsQuery, SearchQuery};
use sidecar_core::{doc_parser, Repository};
use sidecar_types::{Limit, Offset, Uid, Visibility};

use crate::protocol::{JsonRpcRequest, JsonRpcResponse};

const DOC_SCAN_DEFAULT_LIMIT: u32 = 5000;
const DOC_SCAN_PAGE_SIZE: u32 = 200;
const MCP_PROTOCOL_VERSION: &str = "2024-11-05";

/// Dispatch an MCP request to the appropriate tool handler.
pub fn dispatch<R: Repository>(repo: &R, req: &JsonRpcRequest, root: &Path) -> JsonRpcResponse {
    if req.jsonrpc != "2.0" {
        return JsonRpcResponse::error(req.id.clone(), -32600, "Invalid Request".to_owned());
    }

    match req.method.as_str() {
        "initialize" => handle_initialize(req),
        "initialized" | "notifications/initialized" => handle_initialized(req),
        "ping" => handle_ping(req),
        "shutdown" => handle_shutdown(req),
        "tools/list" => handle_tools_list(req),
        "tools/call" => handle_tools_call(repo, req, root),
        _ => dispatch_legacy_tool(repo, req, root).unwrap_or_else(|| {
            JsonRpcResponse::error(
                req.id.clone(),
                -32601,
                format!("Method not found: {}", req.method),
            )
        }),
    }
}

fn dispatch_legacy_tool<R: Repository>(
    repo: &R,
    req: &JsonRpcRequest,
    root: &Path,
) -> Option<JsonRpcResponse> {
    match req.method.as_str() {
        "search_symbols" => Some(handle_search_symbols(repo, req)),
        "get_symbol" => Some(handle_get_symbol(repo, req)),
        "find_references" => Some(handle_find_references(repo, req)),
        "get_documentation" => Some(handle_get_documentation(repo, req, root)),
        "coverage_metrics" => Some(handle_coverage_metrics(repo, req)),
        "detect_undocumented_symbols" => Some(handle_detect_undocumented_symbols(repo, req)),
        _ => None,
    }
}

fn handle_initialize(req: &JsonRpcRequest) -> JsonRpcResponse {
    JsonRpcResponse::success(
        req.id.clone(),
        serde_json::json!({
            "protocolVersion": MCP_PROTOCOL_VERSION,
            "serverInfo": {
                "name": "sidecar-mcp",
                "version": env!("CARGO_PKG_VERSION"),
            },
            "capabilities": {
                "tools": {
                    "listChanged": false
                }
            }
        }),
    )
}

fn handle_initialized(req: &JsonRpcRequest) -> JsonRpcResponse {
    JsonRpcResponse::success(req.id.clone(), serde_json::json!({}))
}

fn handle_ping(req: &JsonRpcRequest) -> JsonRpcResponse {
    JsonRpcResponse::success(req.id.clone(), serde_json::json!({}))
}

fn handle_shutdown(req: &JsonRpcRequest) -> JsonRpcResponse {
    JsonRpcResponse::success(req.id.clone(), serde_json::json!({}))
}

fn handle_tools_list(req: &JsonRpcRequest) -> JsonRpcResponse {
    JsonRpcResponse::success(
        req.id.clone(),
        serde_json::json!({ "tools": mcp_tools_catalog() }),
    )
}

fn handle_tools_call<R: Repository>(
    repo: &R,
    req: &JsonRpcRequest,
    root: &Path,
) -> JsonRpcResponse {
    let tool_name = match parse_required_string(&req.params, "name") {
        Ok(v) => v,
        Err(msg) => return invalid_params(req, &msg),
    };
    let arguments = match parse_optional_object_param(&req.params, "arguments") {
        Ok(v) => v.unwrap_or_else(|| serde_json::json!({})),
        Err(msg) => return invalid_params(req, &msg),
    };

    let tool_req = JsonRpcRequest {
        jsonrpc: req.jsonrpc.clone(),
        id: req.id.clone(),
        method: tool_name.clone(),
        params: arguments,
    };

    let Some(tool_resp) = dispatch_legacy_tool(repo, &tool_req, root) else {
        return JsonRpcResponse::error(
            req.id.clone(),
            -32601,
            format!("Tool not found: {tool_name}"),
        );
    };

    if let Some(err) = tool_resp.error {
        return JsonRpcResponse::error(req.id.clone(), err.code, err.message);
    }

    let Some(result) = tool_resp.result else {
        return internal_error(req, "tool call returned no result");
    };
    let rendered = serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".to_owned());

    JsonRpcResponse::success(
        req.id.clone(),
        serde_json::json!({
            "content": [
                {
                    "type": "text",
                    "text": rendered
                }
            ],
            "structuredContent": result,
            "isError": false
        }),
    )
}

fn mcp_tools_catalog() -> Vec<Value> {
    vec![
        serde_json::json!({
            "name": "search_symbols",
            "description": "Search indexed symbols by name or qualified name.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": {"type": "string"},
                    "limit": {"type": "integer", "minimum": 1, "maximum": 1000},
                    "offset": {"type": "integer", "minimum": 0},
                    "fields": {
                        "oneOf": [
                            {"type": "string"},
                            {"type": "array", "items": {"type": "string"}}
                        ]
                    }
                },
                "required": ["query"]
            }
        }),
        serde_json::json!({
            "name": "get_symbol",
            "description": "Get symbol metadata by UID.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "uid": {"type": "string"},
                    "fields": {
                        "oneOf": [
                            {"type": "string"},
                            {"type": "array", "items": {"type": "string"}}
                        ]
                    }
                },
                "required": ["uid"]
            }
        }),
        serde_json::json!({
            "name": "find_references",
            "description": "Find references to a symbol UID.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "uid": {"type": "string"},
                    "limit": {"type": "integer", "minimum": 1, "maximum": 1000},
                    "offset": {"type": "integer", "minimum": 0},
                    "fields": {
                        "oneOf": [
                            {"type": "string"},
                            {"type": "array", "items": {"type": "string"}}
                        ]
                    }
                },
                "required": ["uid"]
            }
        }),
        serde_json::json!({
            "name": "get_documentation",
            "description": "Get sidecar documentation for a target symbol UID.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "uid": {"type": "string"},
                    "mode": {"type": "string", "enum": ["summary", "full"]},
                    "max_chars": {"type": "integer", "minimum": 1},
                    "fields": {
                        "oneOf": [
                            {"type": "string"},
                            {"type": "array", "items": {"type": "string"}}
                        ]
                    }
                },
                "required": ["uid"]
            }
        }),
        serde_json::json!({
            "name": "coverage_metrics",
            "description": "Compute documentation coverage metrics over indexed symbols.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "public_only": {"type": "boolean"},
                    "scan_limit": {"type": "integer", "minimum": 1}
                }
            }
        }),
        serde_json::json!({
            "name": "detect_undocumented_symbols",
            "description": "Return undocumented symbols in deterministic, paginated order.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "public_only": {"type": "boolean"},
                    "scan_limit": {"type": "integer", "minimum": 1},
                    "limit": {"type": "integer", "minimum": 1, "maximum": 1000},
                    "offset": {"type": "integer", "minimum": 0},
                    "fields": {
                        "oneOf": [
                            {"type": "string"},
                            {"type": "array", "items": {"type": "string"}}
                        ]
                    }
                }
            }
        }),
    ]
}

fn handle_search_symbols<R: Repository>(repo: &R, req: &JsonRpcRequest) -> JsonRpcResponse {
    let query = match parse_required_string(&req.params, "query") {
        Ok(v) => v,
        Err(msg) => return invalid_params(req, &msg),
    };
    let (limit, offset) = match parse_limit_offset(&req.params) {
        Ok(v) => v,
        Err(msg) => return invalid_params(req, &msg),
    };
    let fields = match parse_fields(&req.params) {
        Ok(v) => v,
        Err(msg) => return invalid_params(req, &msg),
    };

    match repo.search_symbols(&SearchQuery {
        query,
        limit,
        offset,
    }) {
        Ok(result) => {
            let results: Vec<Value> = result
                .results
                .into_iter()
                .map(|sym| {
                    select_fields(
                        serde_json::to_value(sym).unwrap_or(Value::Null),
                        fields.as_deref(),
                    )
                })
                .collect();

            JsonRpcResponse::success(
                req.id.clone(),
                serde_json::json!({
                    "results": results,
                    "truncated": result.truncated,
                }),
            )
        }
        Err(e) => internal_error(req, &e.to_string()),
    }
}

fn handle_get_symbol<R: Repository>(repo: &R, req: &JsonRpcRequest) -> JsonRpcResponse {
    let uid = match parse_uid_param(&req.params, "uid") {
        Ok(v) => v,
        Err(msg) => return invalid_params(req, &msg),
    };
    let fields = match parse_fields(&req.params) {
        Ok(v) => v,
        Err(msg) => return invalid_params(req, &msg),
    };

    match repo.get_symbol(&uid) {
        Ok(Some(symbol)) => {
            let symbol_json = select_fields(
                serde_json::to_value(symbol).unwrap_or(Value::Null),
                fields.as_deref(),
            );
            JsonRpcResponse::success(req.id.clone(), serde_json::json!({ "symbol": symbol_json }))
        }
        Ok(None) => JsonRpcResponse::success(req.id.clone(), serde_json::json!({ "symbol": null })),
        Err(e) => internal_error(req, &e.to_string()),
    }
}

fn handle_find_references<R: Repository>(repo: &R, req: &JsonRpcRequest) -> JsonRpcResponse {
    let uid = match parse_uid_param(&req.params, "uid") {
        Ok(v) => v,
        Err(msg) => return invalid_params(req, &msg),
    };
    let (limit, offset) = match parse_limit_offset(&req.params) {
        Ok(v) => v,
        Err(msg) => return invalid_params(req, &msg),
    };
    let fields = match parse_fields(&req.params) {
        Ok(v) => v,
        Err(msg) => return invalid_params(req, &msg),
    };

    match repo.find_refs(&uid, &RefsQuery { limit, offset }) {
        Ok(result) => {
            let refs: Vec<Value> = result
                .results
                .into_iter()
                .map(|r| {
                    select_fields(
                        serde_json::to_value(r).unwrap_or(Value::Null),
                        fields.as_deref(),
                    )
                })
                .collect();
            JsonRpcResponse::success(
                req.id.clone(),
                serde_json::json!({
                    "total": result.total,
                    "results": refs,
                    "truncated": result.truncated,
                }),
            )
        }
        Err(e) => internal_error(req, &e.to_string()),
    }
}

fn handle_get_documentation<R: Repository>(
    repo: &R,
    req: &JsonRpcRequest,
    root: &Path,
) -> JsonRpcResponse {
    let uid = match parse_uid_param(&req.params, "uid") {
        Ok(v) => v,
        Err(msg) => return invalid_params(req, &msg),
    };
    let fields = match parse_fields(&req.params) {
        Ok(v) => v,
        Err(msg) => return invalid_params(req, &msg),
    };
    let mode = match parse_mode(&req.params) {
        Ok(v) => v,
        Err(msg) => return invalid_params(req, &msg),
    };
    let max_chars = match parse_max_chars(&req.params) {
        Ok(v) => v,
        Err(msg) => return invalid_params(req, &msg),
    };

    match repo.get_doc(&uid) {
        Ok(Some(doc)) => {
            let mut summary = doc.summary_cache.clone();
            let mut content: Option<String> = None;
            let mut was_truncated = false;

            if mode == "full" || summary.is_none() {
                let doc_path = root.join(doc.path.as_str());
                let raw = match std::fs::read_to_string(&doc_path) {
                    Ok(v) => v,
                    Err(e) => {
                        return internal_error(req, &format!("cannot read doc file: {e}"));
                    }
                };

                let body = match doc_parser::parse_sidecar_doc(&raw) {
                    Ok((_, body)) => body,
                    Err(_) => raw,
                };

                if summary.is_none() {
                    summary = doc_parser::extract_summary(&body);
                }
                if mode == "full" {
                    content = Some(body);
                }
            }

            if let Some(max) = max_chars {
                if let Some(existing_summary) = summary {
                    let (next_summary, truncated) = truncate_chars(&existing_summary, max);
                    summary = Some(next_summary);
                    was_truncated |= truncated;
                }
                if let Some(existing_content) = content {
                    let (next_content, truncated) = truncate_chars(&existing_content, max);
                    content = Some(next_content);
                    was_truncated |= truncated;
                }
            }

            let mut response = serde_json::json!({
                "exists": true,
                "doc_uid": doc.doc_uid.as_str(),
                "target_uid": doc.target_uid.as_str(),
                "path": doc.path.as_str(),
                "updated_at": doc.updated_at,
                "summary": summary,
                "truncated": was_truncated,
            });

            if mode == "full" {
                response["content"] = serde_json::to_value(content).unwrap_or(Value::Null);
            }

            JsonRpcResponse::success(req.id.clone(), select_fields(response, fields.as_deref()))
        }
        Ok(None) => JsonRpcResponse::success(
            req.id.clone(),
            select_fields(serde_json::json!({"exists": false}), fields.as_deref()),
        ),
        Err(e) => internal_error(req, &e.to_string()),
    }
}

fn handle_coverage_metrics<R: Repository>(repo: &R, req: &JsonRpcRequest) -> JsonRpcResponse {
    let public_only = match parse_public_only(&req.params) {
        Ok(v) => v,
        Err(msg) => return invalid_params(req, &msg),
    };
    let scan_limit = match parse_scan_limit(&req.params) {
        Ok(v) => v,
        Err(msg) => return invalid_params(req, &msg),
    };

    match collect_doc_gap(repo, public_only, scan_limit) {
        Ok(snapshot) => {
            let undocumented_symbols = snapshot.eligible_symbols - snapshot.documented_symbols;
            let coverage_pct =
                round_percentage(snapshot.documented_symbols, snapshot.eligible_symbols);
            JsonRpcResponse::success(
                req.id.clone(),
                serde_json::json!({
                    "public_only": public_only,
                    "scan_limit": scan_limit,
                    "scan_complete": snapshot.scan_complete,
                    "scanned_symbols": snapshot.scanned_symbols,
                    "eligible_symbols": snapshot.eligible_symbols,
                    "documented_symbols": snapshot.documented_symbols,
                    "undocumented_symbols": undocumented_symbols,
                    "coverage_pct": coverage_pct,
                }),
            )
        }
        Err(e) => internal_error(req, &e.to_string()),
    }
}

fn handle_detect_undocumented_symbols<R: Repository>(
    repo: &R,
    req: &JsonRpcRequest,
) -> JsonRpcResponse {
    let (limit, offset) = match parse_limit_offset(&req.params) {
        Ok(v) => v,
        Err(msg) => return invalid_params(req, &msg),
    };
    let public_only = match parse_public_only(&req.params) {
        Ok(v) => v,
        Err(msg) => return invalid_params(req, &msg),
    };
    let scan_limit = match parse_scan_limit(&req.params) {
        Ok(v) => v,
        Err(msg) => return invalid_params(req, &msg),
    };
    let fields = match parse_fields(&req.params) {
        Ok(v) => v,
        Err(msg) => return invalid_params(req, &msg),
    };

    match collect_doc_gap(repo, public_only, scan_limit) {
        Ok(snapshot) => {
            let total = snapshot.undocumented_symbols.len();
            let start = usize::min(offset.value() as usize, total);
            let end = usize::min(start + limit.value() as usize, total);

            let results: Vec<Value> = snapshot.undocumented_symbols[start..end]
                .iter()
                .cloned()
                .map(|sym| {
                    select_fields(
                        serde_json::to_value(sym).unwrap_or(Value::Null),
                        fields.as_deref(),
                    )
                })
                .collect();

            JsonRpcResponse::success(
                req.id.clone(),
                serde_json::json!({
                    "public_only": public_only,
                    "scan_limit": scan_limit,
                    "scan_complete": snapshot.scan_complete,
                    "scanned_symbols": snapshot.scanned_symbols,
                    "eligible_symbols": snapshot.eligible_symbols,
                    "documented_symbols": snapshot.documented_symbols,
                    "undocumented_total": total,
                    "results": results,
                    "truncated": end < total,
                }),
            )
        }
        Err(e) => internal_error(req, &e.to_string()),
    }
}

#[derive(Debug)]
struct DocGapSnapshot {
    scanned_symbols: u32,
    eligible_symbols: u32,
    documented_symbols: u32,
    undocumented_symbols: Vec<Symbol>,
    scan_complete: bool,
}

fn collect_doc_gap<R: Repository>(
    repo: &R,
    public_only: bool,
    scan_limit: u32,
) -> Result<DocGapSnapshot, sidecar_types::SidecarError> {
    let mut scanned_symbols = Vec::new();
    let mut offset = 0u32;
    let mut scan_complete = false;

    while (scanned_symbols.len() as u32) < scan_limit {
        let remaining = scan_limit - scanned_symbols.len() as u32;
        let page_limit = remaining.min(DOC_SCAN_PAGE_SIZE);
        let page = repo.search_symbols(&SearchQuery {
            query: String::new(),
            limit: Limit::new(page_limit).expect("DOC_SCAN_PAGE_SIZE must be a valid limit"),
            offset: Offset::new(offset),
        })?;

        if page.results.is_empty() {
            scan_complete = true;
            break;
        }

        offset = offset.saturating_add(page.results.len() as u32);
        scanned_symbols.extend(page.results);
    }

    let mut eligible_symbols = 0u32;
    let mut documented_symbols = 0u32;
    let mut undocumented_symbols = Vec::new();

    for sym in scanned_symbols.iter().cloned() {
        if public_only && sym.visibility != Visibility::Public {
            continue;
        }
        eligible_symbols = eligible_symbols.saturating_add(1);

        if repo.get_doc(&sym.uid)?.is_some() {
            documented_symbols = documented_symbols.saturating_add(1);
        } else {
            undocumented_symbols.push(sym);
        }
    }

    undocumented_symbols.sort_by(|a, b| a.name.cmp(&b.name).then(a.uid.cmp(&b.uid)));

    Ok(DocGapSnapshot {
        scanned_symbols: scanned_symbols.len() as u32,
        eligible_symbols,
        documented_symbols,
        undocumented_symbols,
        scan_complete,
    })
}

fn invalid_params(req: &JsonRpcRequest, message: &str) -> JsonRpcResponse {
    JsonRpcResponse::error(req.id.clone(), -32602, message.to_owned())
}

fn internal_error(req: &JsonRpcRequest, message: &str) -> JsonRpcResponse {
    JsonRpcResponse::error(req.id.clone(), -32000, message.to_owned())
}

fn parse_required_string(params: &Value, key: &str) -> Result<String, String> {
    match get_param(params, key) {
        Some(Value::String(v)) if !v.trim().is_empty() => Ok(v.to_owned()),
        Some(Value::String(_)) => Err(format!("'{key}' must not be empty")),
        Some(_) => Err(format!("'{key}' must be a string")),
        None => Err(format!("missing required parameter '{key}'")),
    }
}

fn parse_uid_param(params: &Value, key: &str) -> Result<Uid, String> {
    let raw = parse_required_string(params, key)?;
    raw.parse::<Uid>()
        .map_err(|e| format!("invalid UID in '{key}': {e}"))
}

fn parse_optional_u32(params: &Value, key: &str) -> Result<Option<u32>, String> {
    match get_param(params, key) {
        None => Ok(None),
        Some(Value::Number(n)) => {
            let raw = n
                .as_u64()
                .ok_or_else(|| format!("'{key}' must be a non-negative integer"))?;
            let value = u32::try_from(raw)
                .map_err(|_| format!("'{key}' exceeds max value {}", u32::MAX))?;
            Ok(Some(value))
        }
        Some(_) => Err(format!("'{key}' must be a non-negative integer")),
    }
}

fn parse_limit_offset(params: &Value) -> Result<(Limit, Offset), String> {
    let limit = match parse_optional_u32(params, "limit")? {
        Some(n) => Limit::new(n).map_err(|e| e.to_string())?,
        None => Limit::default(),
    };
    let offset = Offset::new(parse_optional_u32(params, "offset")?.unwrap_or(0));
    Ok((limit, offset))
}

fn parse_mode(params: &Value) -> Result<String, String> {
    match get_param(params, "mode") {
        None => Ok("summary".to_owned()),
        Some(Value::String(mode)) if mode == "summary" || mode == "full" => Ok(mode.to_owned()),
        Some(Value::String(_)) => Err("mode must be one of: summary, full".to_owned()),
        Some(_) => Err("'mode' must be a string".to_owned()),
    }
}

fn parse_max_chars(params: &Value) -> Result<Option<usize>, String> {
    match parse_optional_u32(params, "max_chars")? {
        Some(0) => Err("'max_chars' must be at least 1 when provided".to_owned()),
        Some(v) => Ok(Some(v as usize)),
        None => Ok(None),
    }
}

fn parse_public_only(params: &Value) -> Result<bool, String> {
    match get_param(params, "public_only") {
        None => Ok(true),
        Some(Value::Bool(v)) => Ok(*v),
        Some(_) => Err("'public_only' must be a boolean".to_owned()),
    }
}

fn parse_scan_limit(params: &Value) -> Result<u32, String> {
    match parse_optional_u32(params, "scan_limit")? {
        None => Ok(DOC_SCAN_DEFAULT_LIMIT),
        Some(0) => Err("'scan_limit' must be at least 1 when provided".to_owned()),
        Some(v) => Ok(v),
    }
}

fn parse_fields(params: &Value) -> Result<Option<Vec<String>>, String> {
    match get_param(params, "fields") {
        None => Ok(None),
        Some(Value::String(s)) => {
            let fields: Vec<String> = s
                .split(',')
                .map(str::trim)
                .filter(|field| !field.is_empty())
                .map(ToOwned::to_owned)
                .collect();
            if fields.is_empty() {
                Err("'fields' must contain at least one field".to_owned())
            } else {
                Ok(Some(fields))
            }
        }
        Some(Value::Array(arr)) => {
            let mut fields = Vec::with_capacity(arr.len());
            for item in arr {
                match item {
                    Value::String(s) if !s.trim().is_empty() => fields.push(s.trim().to_owned()),
                    Value::String(_) => return Err("'fields' items must not be empty".to_owned()),
                    _ => return Err("'fields' must be an array of strings".to_owned()),
                }
            }
            if fields.is_empty() {
                Err("'fields' must contain at least one field".to_owned())
            } else {
                Ok(Some(fields))
            }
        }
        Some(_) => Err("'fields' must be a comma-separated string or string array".to_owned()),
    }
}

fn parse_optional_object_param(params: &Value, key: &str) -> Result<Option<Value>, String> {
    match get_param(params, key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Object(obj)) => Ok(Some(Value::Object(obj.clone()))),
        Some(_) => Err(format!("'{key}' must be an object")),
    }
}

fn get_param<'a>(params: &'a Value, key: &str) -> Option<&'a Value> {
    params.as_object().and_then(|map| map.get(key))
}

fn select_fields(value: Value, fields: Option<&[String]>) -> Value {
    let Some(fields) = fields else {
        return value;
    };

    match value {
        Value::Object(obj) => {
            let mut filtered = serde_json::Map::with_capacity(fields.len());
            for field in fields {
                if let Some(v) = obj.get(field) {
                    filtered.insert(field.clone(), v.clone());
                }
            }
            Value::Object(filtered)
        }
        other => other,
    }
}

fn truncate_chars(value: &str, max_chars: usize) -> (String, bool) {
    let total_chars = value.chars().count();
    if total_chars <= max_chars {
        (value.to_owned(), false)
    } else {
        (value.chars().take(max_chars).collect(), true)
    }
}

fn round_percentage(numerator: u32, denominator: u32) -> f64 {
    if denominator == 0 {
        100.0
    } else {
        ((numerator as f64 / denominator as f64) * 10000.0).round() / 100.0
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::Path;

    use insta::assert_yaml_snapshot;
    use sidecar_core::model::{DocRecord, FileRecord, Reference, Symbol};
    use sidecar_core::query::{RefsQuery, RefsResult, SearchQuery, SearchResult};
    use sidecar_core::Repository;
    use sidecar_types::{
        ContentHash, Fingerprint, Language, PathRel, Range, RefKind, SidecarError, SymbolKind, Uid,
        Visibility,
    };

    use super::*;

    #[derive(Default)]
    struct MockRepo {
        search_results: Vec<Symbol>,
        search_truncated: bool,
        symbol: Option<Symbol>,
        refs_total: u32,
        refs_results: Vec<Reference>,
        refs_truncated: bool,
        doc: Option<DocRecord>,
        docs_by_target: BTreeMap<Uid, DocRecord>,
    }

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

        fn search_symbols(&self, query: &SearchQuery) -> Result<SearchResult, SidecarError> {
            let normalized_query = query.query.to_lowercase();
            let mut matched: Vec<Symbol> = self
                .search_results
                .iter()
                .filter(|sym| {
                    if normalized_query.is_empty() {
                        true
                    } else {
                        sym.name.to_lowercase().contains(&normalized_query)
                            || sym
                                .qualified_name
                                .to_lowercase()
                                .contains(&normalized_query)
                    }
                })
                .cloned()
                .collect();
            matched.sort_by(|a, b| a.name.cmp(&b.name).then(a.uid.cmp(&b.uid)));

            let start = usize::min(query.offset.value() as usize, matched.len());
            let end = usize::min(start + query.limit.value() as usize, matched.len());
            let results = matched[start..end].to_vec();

            Ok(SearchResult {
                results,
                truncated: self.search_truncated || end < matched.len(),
            })
        }

        fn get_symbol(&self, _uid: &Uid) -> Result<Option<Symbol>, SidecarError> {
            Ok(self.symbol.clone())
        }

        fn find_refs(&self, _uid: &Uid, _query: &RefsQuery) -> Result<RefsResult, SidecarError> {
            Ok(RefsResult {
                total: self.refs_total,
                results: self.refs_results.clone(),
                truncated: self.refs_truncated,
            })
        }

        fn get_doc(&self, uid: &Uid) -> Result<Option<DocRecord>, SidecarError> {
            if let Some(doc) = self.docs_by_target.get(uid) {
                return Ok(Some(doc.clone()));
            }
            Ok(self.doc.clone())
        }

        fn upsert_docs(&self, _docs: &[DocRecord]) -> Result<(), SidecarError> {
            Ok(())
        }
    }

    fn sample_symbol() -> Symbol {
        Symbol {
            uid: "sym:ts:src/cart:CartService:866eb7ea".parse().unwrap(),
            file_uid: "file:src/cart.ts".parse().unwrap(),
            kind: SymbolKind::Class,
            qualified_name: "CartService".to_owned(),
            name: "CartService".to_owned(),
            visibility: Visibility::Public,
            fingerprint: Fingerprint::from_hex(
                "866eb7eaac9f2ac8c705957c84f836446525b1d4af48bf3d8034a68680e09c9c".to_owned(),
            ),
            range: Range {
                start: 10,
                end: 100,
            },
        }
    }

    fn sample_symbol_with(uid: &str, name: &str, visibility: Visibility) -> Symbol {
        Symbol {
            uid: uid.parse().unwrap(),
            file_uid: "file:src/cart.ts".parse().unwrap(),
            kind: SymbolKind::Class,
            qualified_name: name.to_owned(),
            name: name.to_owned(),
            visibility,
            fingerprint: Fingerprint::from_hex(
                "866eb7eaac9f2ac8c705957c84f836446525b1d4af48bf3d8034a68680e09c9c".to_owned(),
            ),
            range: Range {
                start: 10,
                end: 100,
            },
        }
    }

    fn sample_ref() -> Reference {
        Reference {
            from_uid: "sym:ts:src/cart:createCart:cb65d33e".parse().unwrap(),
            to_uid: "sym:ts:src/cart:CartService:866eb7ea".parse().unwrap(),
            file_uid: "file:src/cart.ts".parse().unwrap(),
            range: Range { start: 50, end: 60 },
            ref_kind: RefKind::TypeRef,
        }
    }

    fn sample_doc(path: &str) -> DocRecord {
        DocRecord {
            doc_uid: "doc:cart-overview".parse().unwrap(),
            target_uid: "sym:ts:src/cart:CartService:866eb7ea".parse().unwrap(),
            path: path.parse().unwrap(),
            summary_cache: Some("Cached summary".to_owned()),
            updated_at: "2026-01-01T00:00:00Z".to_owned(),
        }
    }

    fn sample_file() -> FileRecord {
        FileRecord {
            file_uid: "file:src/cart.ts".parse().unwrap(),
            path: "src/cart.ts".parse().unwrap(),
            language: Language::TypeScript,
            content_hash: ContentHash::from_hex(
                "4f6ccf7f647ce84b88f72185f7f00fd9f235f28f34dd30a2cbf95d8f6f64e592".to_owned(),
            ),
            last_indexed_at: "2026-01-01T00:00:00Z".to_owned(),
        }
    }

    struct ErrorRepo;

    impl Repository for ErrorRepo {
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
            Err(SidecarError::Index("forced search failure".to_owned()))
        }

        fn get_symbol(&self, _uid: &Uid) -> Result<Option<Symbol>, SidecarError> {
            Err(SidecarError::Index("forced symbol failure".to_owned()))
        }

        fn find_refs(&self, _uid: &Uid, _query: &RefsQuery) -> Result<RefsResult, SidecarError> {
            Err(SidecarError::Index("forced refs failure".to_owned()))
        }

        fn get_doc(&self, _uid: &Uid) -> Result<Option<DocRecord>, SidecarError> {
            Err(SidecarError::Index("forced doc failure".to_owned()))
        }

        fn upsert_docs(&self, _docs: &[DocRecord]) -> Result<(), SidecarError> {
            Ok(())
        }
    }

    #[test]
    fn rejects_invalid_jsonrpc_version() {
        let repo = MockRepo::default();
        let req = JsonRpcRequest {
            jsonrpc: "1.0".to_owned(),
            id: serde_json::json!(1),
            method: "search_symbols".to_owned(),
            params: serde_json::json!({"query":"Cart"}),
        };

        let resp = dispatch(&repo, &req, Path::new("."));
        assert!(resp.result.is_none());
        assert_eq!(resp.error.unwrap().code, -32600);
    }

    #[test]
    fn rejects_unknown_method() {
        let repo = MockRepo::default();
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(99),
            method: "not_a_method".to_owned(),
            params: serde_json::json!({}),
        };

        let resp = dispatch(&repo, &req, Path::new("."));
        assert!(resp.result.is_none());
        let err = resp.error.unwrap();
        assert_eq!(err.code, -32601);
        assert!(err.message.contains("Method not found"));
    }

    #[test]
    fn initialize_returns_mcp_capabilities() {
        let repo = MockRepo::default();
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(100),
            method: "initialize".to_owned(),
            params: serde_json::json!({
                "protocolVersion": "2024-11-05"
            }),
        };

        let resp = dispatch(&repo, &req, Path::new("."));
        assert!(resp.error.is_none());
        let result = resp.result.unwrap();
        assert_eq!(result["protocolVersion"], MCP_PROTOCOL_VERSION);
        assert_eq!(result["serverInfo"]["name"], "sidecar-mcp");
        assert_eq!(result["capabilities"]["tools"]["listChanged"], false);
    }

    #[test]
    fn tools_list_returns_registered_tools() {
        let repo = MockRepo::default();
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(101),
            method: "tools/list".to_owned(),
            params: serde_json::json!({}),
        };

        let resp = dispatch(&repo, &req, Path::new("."));
        assert!(resp.error.is_none());
        let result = resp.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert!(tools.iter().any(|t| t["name"] == "search_symbols"));
        assert!(tools.iter().any(|t| t["name"] == "coverage_metrics"));
        assert!(tools
            .iter()
            .any(|t| t["name"] == "detect_undocumented_symbols"));
    }

    #[test]
    fn tools_call_wraps_structured_content() {
        let repo = MockRepo {
            search_results: vec![sample_symbol()],
            ..Default::default()
        };
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(102),
            method: "tools/call".to_owned(),
            params: serde_json::json!({
                "name": "search_symbols",
                "arguments": {
                    "query":"CartService",
                    "limit": 1
                }
            }),
        };

        let resp = dispatch(&repo, &req, Path::new("."));
        assert!(resp.error.is_none());
        let result = resp.result.unwrap();
        assert_eq!(result["isError"], false);
        assert_eq!(
            result["structuredContent"]["results"][0]["name"],
            "CartService"
        );
        assert_eq!(result["content"][0]["type"], "text");
        assert!(result["content"][0]["text"]
            .as_str()
            .unwrap()
            .contains("CartService"));
    }

    #[test]
    fn tools_call_validates_params_and_tool_name() {
        let repo = MockRepo::default();

        let missing_name = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(103),
            method: "tools/call".to_owned(),
            params: serde_json::json!({}),
        };
        let resp = dispatch(&repo, &missing_name, Path::new("."));
        assert_eq!(resp.error.unwrap().code, -32602);

        let invalid_args_type = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(104),
            method: "tools/call".to_owned(),
            params: serde_json::json!({
                "name": "search_symbols",
                "arguments": "bad"
            }),
        };
        let resp = dispatch(&repo, &invalid_args_type, Path::new("."));
        assert_eq!(resp.error.unwrap().code, -32602);

        let unknown_tool = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(105),
            method: "tools/call".to_owned(),
            params: serde_json::json!({
                "name": "does_not_exist",
                "arguments": {}
            }),
        };
        let resp = dispatch(&repo, &unknown_tool, Path::new("."));
        assert_eq!(resp.error.unwrap().code, -32601);
    }

    #[test]
    fn search_symbols_applies_field_selection() {
        let repo = MockRepo {
            search_results: vec![sample_symbol()],
            ..Default::default()
        };
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(1),
            method: "search_symbols".to_owned(),
            params: serde_json::json!({
                "query":"CartService",
                "fields":["uid","name"],
            }),
        };

        let resp = dispatch(&repo, &req, Path::new("."));
        assert!(resp.error.is_none());
        let result = resp.result.unwrap();
        assert_eq!(
            result["results"][0]["uid"],
            "sym:ts:src/cart:CartService:866eb7ea"
        );
        assert_eq!(result["results"][0]["name"], "CartService");
        assert!(result["results"][0].get("kind").is_none());
    }

    #[test]
    fn search_symbols_validates_params() {
        let repo = MockRepo::default();

        let missing_query = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(1),
            method: "search_symbols".to_owned(),
            params: serde_json::json!({"limit": 10}),
        };
        let resp = dispatch(&repo, &missing_query, Path::new("."));
        assert_eq!(resp.error.unwrap().code, -32602);

        let invalid_limit = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(2),
            method: "search_symbols".to_owned(),
            params: serde_json::json!({"query": "x", "limit": "nope"}),
        };
        let resp = dispatch(&repo, &invalid_limit, Path::new("."));
        assert_eq!(resp.error.unwrap().code, -32602);

        let invalid_fields = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(3),
            method: "search_symbols".to_owned(),
            params: serde_json::json!({"query":"x", "fields": ["uid", 1]}),
        };
        let resp = dispatch(&repo, &invalid_fields, Path::new("."));
        assert_eq!(resp.error.unwrap().code, -32602);
    }

    #[test]
    fn coverage_metrics_reports_public_doc_coverage() {
        let documented = sample_symbol_with(
            "sym:ts:src/cart:ADocumented:11111111",
            "ADocumented",
            Visibility::Public,
        );
        let undocumented = sample_symbol_with(
            "sym:ts:src/cart:BUndocumented:22222222",
            "BUndocumented",
            Visibility::Public,
        );
        let private = sample_symbol_with(
            "sym:ts:src/cart:CPrivate:33333333",
            "CPrivate",
            Visibility::Private,
        );

        let mut docs_by_target = BTreeMap::new();
        docs_by_target.insert(
            documented.uid.clone(),
            DocRecord {
                target_uid: documented.uid.clone(),
                ..sample_doc("docs-sidecar/symbols/doc-a.md")
            },
        );

        let repo = MockRepo {
            search_results: vec![undocumented.clone(), private, documented.clone()],
            docs_by_target,
            ..Default::default()
        };
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(10),
            method: "coverage_metrics".to_owned(),
            params: serde_json::json!({"public_only": true, "scan_limit": 50}),
        };

        let resp = dispatch(&repo, &req, Path::new("."));
        assert!(resp.error.is_none());
        let result = resp.result.unwrap();
        assert_eq!(result["eligible_symbols"], 2);
        assert_eq!(result["documented_symbols"], 1);
        assert_eq!(result["undocumented_symbols"], 1);
        assert_eq!(result["coverage_pct"], 50.0);
        assert_eq!(result["scan_complete"], true);
    }

    #[test]
    fn detect_undocumented_symbols_returns_paginated_results() {
        let a = sample_symbol_with(
            "sym:ts:src/cart:Alpha:aaaaaaaa",
            "Alpha",
            Visibility::Public,
        );
        let b = sample_symbol_with("sym:ts:src/cart:Beta:bbbbbbbb", "Beta", Visibility::Public);
        let c = sample_symbol_with(
            "sym:ts:src/cart:Gamma:cccccccc",
            "Gamma",
            Visibility::Public,
        );

        let mut docs_by_target = BTreeMap::new();
        docs_by_target.insert(
            b.uid.clone(),
            DocRecord {
                target_uid: b.uid.clone(),
                ..sample_doc("docs-sidecar/symbols/doc-b.md")
            },
        );

        let repo = MockRepo {
            search_results: vec![c.clone(), a.clone(), b.clone()],
            docs_by_target,
            ..Default::default()
        };
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(11),
            method: "detect_undocumented_symbols".to_owned(),
            params: serde_json::json!({
                "limit": 1,
                "offset": 1,
                "fields": ["uid", "name"],
                "public_only": true,
                "scan_limit": 50
            }),
        };

        let resp = dispatch(&repo, &req, Path::new("."));
        assert!(resp.error.is_none());
        let result = resp.result.unwrap();
        assert_eq!(result["undocumented_total"], 2);
        assert_eq!(result["results"][0]["uid"], c.uid.as_str());
        assert_eq!(result["results"][0]["name"], "Gamma");
        assert!(result["results"][0]["kind"].is_null());
        assert_eq!(result["truncated"], false);
    }

    #[test]
    fn doc_gap_tools_validate_params() {
        let repo = MockRepo::default();

        let invalid_public_only = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(12),
            method: "coverage_metrics".to_owned(),
            params: serde_json::json!({"public_only": "yes"}),
        };
        let resp = dispatch(&repo, &invalid_public_only, Path::new("."));
        assert_eq!(resp.error.unwrap().code, -32602);

        let invalid_scan_limit = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(13),
            method: "detect_undocumented_symbols".to_owned(),
            params: serde_json::json!({"scan_limit": 0}),
        };
        let resp = dispatch(&repo, &invalid_scan_limit, Path::new("."));
        assert_eq!(resp.error.unwrap().code, -32602);
    }

    #[test]
    fn find_references_rejects_invalid_limit() {
        let repo = MockRepo::default();
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(1),
            method: "find_references".to_owned(),
            params: serde_json::json!({
                "uid":"sym:ts:src/cart:CartService:866eb7ea",
                "limit":0
            }),
        };

        let resp = dispatch(&repo, &req, Path::new("."));
        assert!(resp.result.is_none());
        assert_eq!(resp.error.unwrap().code, -32602);
    }

    #[test]
    fn find_references_returns_results() {
        let repo = MockRepo {
            refs_total: 1,
            refs_results: vec![sample_ref()],
            ..Default::default()
        };
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(1),
            method: "find_references".to_owned(),
            params: serde_json::json!({
                "uid":"sym:ts:src/cart:CartService:866eb7ea"
            }),
        };

        let resp = dispatch(&repo, &req, Path::new("."));
        assert!(resp.error.is_none());
        let result = resp.result.unwrap();
        assert_eq!(result["total"], 1);
        assert_eq!(result["results"][0]["ref_kind"], "type_ref");
    }

    #[test]
    fn find_references_validates_uid_and_fields() {
        let repo = MockRepo::default();
        let invalid_uid = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(1),
            method: "find_references".to_owned(),
            params: serde_json::json!({
                "uid":"bad uid"
            }),
        };
        let resp = dispatch(&repo, &invalid_uid, Path::new("."));
        assert_eq!(resp.error.unwrap().code, -32602);

        let invalid_fields = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(2),
            method: "find_references".to_owned(),
            params: serde_json::json!({
                "uid":"sym:ts:src/cart:CartService:866eb7ea",
                "fields": 1
            }),
        };
        let resp = dispatch(&repo, &invalid_fields, Path::new("."));
        assert_eq!(resp.error.unwrap().code, -32602);
    }

    #[test]
    fn get_documentation_returns_cached_summary() {
        let repo = MockRepo {
            doc: Some(sample_doc("docs-sidecar/symbols/doc-cart.md")),
            ..Default::default()
        };
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(1),
            method: "get_documentation".to_owned(),
            params: serde_json::json!({
                "uid":"sym:ts:src/cart:CartService:866eb7ea",
                "mode":"summary"
            }),
        };

        let resp = dispatch(&repo, &req, Path::new("."));
        assert!(resp.error.is_none());
        let result = resp.result.unwrap();
        assert_eq!(result["exists"], true);
        assert_eq!(result["summary"], "Cached summary");
        assert!(result.get("content").is_none());
    }

    #[test]
    fn get_documentation_full_reads_and_truncates_body() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        let doc_rel = "docs-sidecar/symbols/doc-cart.md";
        let doc_path = root.join(doc_rel);
        fs::create_dir_all(doc_path.parent().unwrap()).unwrap();
        fs::write(
            &doc_path,
            r#"---
doc_uid: doc:cart-overview
title: CartService Overview
anchors:
  - anchor_type: symbol
    symbol_uid: sym:ts:src/cart:CartService:866eb7ea
---

## Overview

This is a long overview body for deterministic truncation tests.
"#,
        )
        .unwrap();

        let repo = MockRepo {
            doc: Some(DocRecord {
                summary_cache: None,
                ..sample_doc(doc_rel)
            }),
            ..Default::default()
        };
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(1),
            method: "get_documentation".to_owned(),
            params: serde_json::json!({
                "uid":"sym:ts:src/cart:CartService:866eb7ea",
                "mode":"full",
                "max_chars":24
            }),
        };

        let resp = dispatch(&repo, &req, root);
        assert!(resp.error.is_none());
        let result = resp.result.unwrap();
        assert_eq!(result["exists"], true);
        assert_eq!(result["truncated"], true);
        assert_eq!(result["content"].as_str().unwrap().chars().count(), 24);
    }

    #[test]
    fn get_symbol_handles_not_found_and_validation_errors() {
        let repo = MockRepo::default();
        let not_found = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(1),
            method: "get_symbol".to_owned(),
            params: serde_json::json!({
                "uid":"sym:ts:src/cart:Missing:abcd1234"
            }),
        };
        let resp = dispatch(&repo, &not_found, Path::new("."));
        assert!(resp.error.is_none());
        assert!(resp.result.unwrap()["symbol"].is_null());

        let invalid_uid = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(2),
            method: "get_symbol".to_owned(),
            params: serde_json::json!({
                "uid":"bad uid"
            }),
        };
        let resp = dispatch(&repo, &invalid_uid, Path::new("."));
        assert_eq!(resp.error.unwrap().code, -32602);

        let invalid_fields = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(3),
            method: "get_symbol".to_owned(),
            params: serde_json::json!({
                "uid":"sym:ts:src/cart:CartService:866eb7ea",
                "fields":" , "
            }),
        };
        let resp = dispatch(&repo, &invalid_fields, Path::new("."));
        assert_eq!(resp.error.unwrap().code, -32602);
    }

    #[test]
    fn get_documentation_covers_not_found_validation_and_read_errors() {
        let repo = MockRepo::default();

        let not_found = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(1),
            method: "get_documentation".to_owned(),
            params: serde_json::json!({
                "uid":"sym:ts:src/cart:CartService:866eb7ea"
            }),
        };
        let resp = dispatch(&repo, &not_found, Path::new("."));
        assert!(resp.error.is_none());
        assert_eq!(resp.result.unwrap()["exists"], false);

        let invalid_mode = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(2),
            method: "get_documentation".to_owned(),
            params: serde_json::json!({
                "uid":"sym:ts:src/cart:CartService:866eb7ea",
                "mode":"raw"
            }),
        };
        let resp = dispatch(&repo, &invalid_mode, Path::new("."));
        assert_eq!(resp.error.unwrap().code, -32602);

        let invalid_mode_type = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(3),
            method: "get_documentation".to_owned(),
            params: serde_json::json!({
                "uid":"sym:ts:src/cart:CartService:866eb7ea",
                "mode": 1
            }),
        };
        let resp = dispatch(&repo, &invalid_mode_type, Path::new("."));
        assert_eq!(resp.error.unwrap().code, -32602);

        let invalid_max_chars = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(4),
            method: "get_documentation".to_owned(),
            params: serde_json::json!({
                "uid":"sym:ts:src/cart:CartService:866eb7ea",
                "max_chars": 0
            }),
        };
        let resp = dispatch(&repo, &invalid_max_chars, Path::new("."));
        assert_eq!(resp.error.unwrap().code, -32602);

        let temp = tempfile::tempdir().unwrap();
        let missing_doc_repo = MockRepo {
            doc: Some(sample_doc("docs-sidecar/missing.md")),
            ..Default::default()
        };
        let read_err = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(5),
            method: "get_documentation".to_owned(),
            params: serde_json::json!({
                "uid":"sym:ts:src/cart:CartService:866eb7ea",
                "mode":"full"
            }),
        };
        let resp = dispatch(&missing_doc_repo, &read_err, temp.path());
        assert!(resp.result.is_none());
        assert_eq!(resp.error.unwrap().code, -32000);
    }

    #[test]
    fn propagates_internal_errors_from_repository() {
        let repo = ErrorRepo;
        let root = Path::new(".");

        let search_req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(1),
            method: "search_symbols".to_owned(),
            params: serde_json::json!({"query":"Cart"}),
        };
        assert_eq!(
            dispatch(&repo, &search_req, root).error.unwrap().code,
            -32000
        );

        let symbol_req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(2),
            method: "get_symbol".to_owned(),
            params: serde_json::json!({"uid":"sym:ts:src/cart:CartService:866eb7ea"}),
        };
        assert_eq!(
            dispatch(&repo, &symbol_req, root).error.unwrap().code,
            -32000
        );

        let refs_req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(3),
            method: "find_references".to_owned(),
            params: serde_json::json!({"uid":"sym:ts:src/cart:CartService:866eb7ea"}),
        };
        assert_eq!(dispatch(&repo, &refs_req, root).error.unwrap().code, -32000);

        let doc_req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(4),
            method: "get_documentation".to_owned(),
            params: serde_json::json!({"uid":"sym:ts:src/cart:CartService:866eb7ea"}),
        };
        assert_eq!(dispatch(&repo, &doc_req, root).error.unwrap().code, -32000);
    }

    #[test]
    fn helper_parsers_cover_edge_cases() {
        assert_eq!(
            parse_required_string(&serde_json::json!({"query":"x"}), "query").unwrap(),
            "x"
        );
        assert!(parse_required_string(&serde_json::json!({"query":""}), "query").is_err());
        assert!(parse_required_string(&serde_json::json!({"query":1}), "query").is_err());
        assert!(parse_required_string(&serde_json::json!({}), "query").is_err());

        assert!(parse_optional_u32(&serde_json::json!({"n":-1}), "n").is_err());
        assert!(parse_optional_u32(&serde_json::json!({"n":"x"}), "n").is_err());
        assert!(parse_optional_u32(&serde_json::json!({}), "n")
            .unwrap()
            .is_none());

        assert_eq!(
            parse_mode(&serde_json::json!({})).unwrap(),
            "summary".to_owned()
        );
        assert!(parse_mode(&serde_json::json!({"mode":"raw"})).is_err());
        assert!(parse_mode(&serde_json::json!({"mode":1})).is_err());
        assert!(parse_max_chars(&serde_json::json!({"max_chars":0})).is_err());
        assert!(parse_public_only(&serde_json::json!({"public_only":"yes"})).is_err());
        assert_eq!(parse_public_only(&serde_json::json!({})).unwrap(), true);
        assert_eq!(
            parse_scan_limit(&serde_json::json!({})).unwrap(),
            DOC_SCAN_DEFAULT_LIMIT
        );
        assert!(parse_scan_limit(&serde_json::json!({"scan_limit":0})).is_err());

        assert_eq!(
            parse_fields(&serde_json::json!({"fields":"uid,name"}))
                .unwrap()
                .unwrap(),
            vec!["uid".to_owned(), "name".to_owned()]
        );
        assert!(parse_fields(&serde_json::json!({"fields":" , " })).is_err());
        assert_eq!(
            parse_fields(&serde_json::json!({"fields":["uid"," name "]}))
                .unwrap()
                .unwrap(),
            vec!["uid".to_owned(), "name".to_owned()]
        );
        assert!(parse_fields(&serde_json::json!({"fields":[""]})).is_err());
        assert!(parse_fields(&serde_json::json!({"fields":[1]})).is_err());
        assert!(parse_fields(&serde_json::json!({"fields":[] })).is_err());
        assert!(parse_fields(&serde_json::json!({"fields":1})).is_err());

        assert!(parse_optional_object_param(&serde_json::json!({"a":"x"}), "a").is_err());
        assert!(parse_optional_object_param(&serde_json::json!({}), "a")
            .unwrap()
            .is_none());
        assert_eq!(
            parse_optional_object_param(&serde_json::json!({"a":{"k":"v"}}), "a").unwrap(),
            Some(serde_json::json!({"k":"v"}))
        );

        assert_eq!(
            select_fields(
                serde_json::json!({"uid":"u","name":"n","kind":"class"}),
                Some(&["uid".to_owned(), "name".to_owned()])
            ),
            serde_json::json!({"uid":"u","name":"n"})
        );
        assert_eq!(
            select_fields(Value::String("x".to_owned()), Some(&["uid".to_owned()])),
            Value::String("x".to_owned())
        );

        assert_eq!(truncate_chars("abc", 5), ("abc".to_owned(), false));
        assert_eq!(truncate_chars("abcdef", 3), ("abc".to_owned(), true));
        assert_eq!(round_percentage(1, 2), 50.0);
        assert_eq!(round_percentage(0, 0), 100.0);
    }

    #[test]
    fn mock_repo_noop_methods_are_callable() {
        let repo = MockRepo::default();
        repo.upsert_file(&sample_file()).unwrap();
        repo.upsert_symbols(&[sample_symbol()]).unwrap();
        repo.upsert_refs(&[sample_ref()]).unwrap();
        assert!(repo
            .get_file_by_path(&"src/cart.ts".parse().unwrap())
            .unwrap()
            .is_none());
        repo.upsert_docs(&[sample_doc("docs-sidecar/symbols/doc-cart.md")])
            .unwrap();
    }

    #[test]
    fn snapshot_tools_list_response() {
        let repo = MockRepo::default();
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(41),
            method: "tools/list".to_owned(),
            params: serde_json::json!({}),
        };

        let resp = dispatch(&repo, &req, Path::new("."));
        assert_yaml_snapshot!("tools_list_response", resp);
    }

    #[test]
    fn snapshot_tools_call_response() {
        let repo = MockRepo {
            search_results: vec![sample_symbol()],
            ..Default::default()
        };
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(40),
            method: "tools/call".to_owned(),
            params: serde_json::json!({
                "name": "search_symbols",
                "arguments": {
                    "query":"CartService",
                    "limit": 1
                }
            }),
        };

        let resp = dispatch(&repo, &req, Path::new("."));
        assert_yaml_snapshot!("tools_call_response", resp);
    }

    #[test]
    fn snapshot_search_symbols_response() {
        let repo = MockRepo {
            search_results: vec![sample_symbol()],
            ..Default::default()
        };
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(42),
            method: "search_symbols".to_owned(),
            params: serde_json::json!({
                "query":"CartService",
                "limit":1
            }),
        };

        let resp = dispatch(&repo, &req, Path::new("."));
        assert_yaml_snapshot!("search_symbols_response", resp);
    }

    #[test]
    fn snapshot_get_symbol_response() {
        let repo = MockRepo {
            symbol: Some(sample_symbol()),
            ..Default::default()
        };
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(7),
            method: "get_symbol".to_owned(),
            params: serde_json::json!({
                "uid":"sym:ts:src/cart:CartService:866eb7ea"
            }),
        };

        let resp = dispatch(&repo, &req, Path::new("."));
        assert_yaml_snapshot!("get_symbol_response", resp);
    }

    #[test]
    fn snapshot_find_references_response() {
        let repo = MockRepo {
            refs_total: 1,
            refs_results: vec![sample_ref()],
            ..Default::default()
        };
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(8),
            method: "find_references".to_owned(),
            params: serde_json::json!({
                "uid":"sym:ts:src/cart:CartService:866eb7ea"
            }),
        };

        let resp = dispatch(&repo, &req, Path::new("."));
        assert_yaml_snapshot!("find_references_response", resp);
    }

    #[test]
    fn snapshot_get_documentation_response() {
        let repo = MockRepo {
            doc: Some(sample_doc("docs-sidecar/symbols/doc-cart.md")),
            ..Default::default()
        };
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(9),
            method: "get_documentation".to_owned(),
            params: serde_json::json!({
                "uid":"sym:ts:src/cart:CartService:866eb7ea",
                "mode":"summary"
            }),
        };

        let resp = dispatch(&repo, &req, Path::new("."));
        assert_yaml_snapshot!("get_documentation_response", resp);
    }

    #[test]
    fn snapshot_coverage_metrics_response() {
        let documented = sample_symbol_with(
            "sym:ts:src/cart:ADocumented:11111111",
            "ADocumented",
            Visibility::Public,
        );
        let undocumented = sample_symbol_with(
            "sym:ts:src/cart:BUndocumented:22222222",
            "BUndocumented",
            Visibility::Public,
        );
        let mut docs_by_target = BTreeMap::new();
        docs_by_target.insert(
            documented.uid.clone(),
            DocRecord {
                target_uid: documented.uid.clone(),
                ..sample_doc("docs-sidecar/symbols/doc-a.md")
            },
        );
        let repo = MockRepo {
            search_results: vec![documented, undocumented],
            docs_by_target,
            ..Default::default()
        };
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(31),
            method: "coverage_metrics".to_owned(),
            params: serde_json::json!({}),
        };

        let resp = dispatch(&repo, &req, Path::new("."));
        assert_yaml_snapshot!("coverage_metrics_response", resp);
    }

    #[test]
    fn snapshot_detect_undocumented_symbols_response() {
        let documented = sample_symbol_with(
            "sym:ts:src/cart:ADocumented:11111111",
            "ADocumented",
            Visibility::Public,
        );
        let undocumented = sample_symbol_with(
            "sym:ts:src/cart:BUndocumented:22222222",
            "BUndocumented",
            Visibility::Public,
        );
        let mut docs_by_target = BTreeMap::new();
        docs_by_target.insert(
            documented.uid.clone(),
            DocRecord {
                target_uid: documented.uid.clone(),
                ..sample_doc("docs-sidecar/symbols/doc-a.md")
            },
        );
        let repo = MockRepo {
            search_results: vec![documented, undocumented],
            docs_by_target,
            ..Default::default()
        };
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_owned(),
            id: serde_json::json!(32),
            method: "detect_undocumented_symbols".to_owned(),
            params: serde_json::json!({"limit": 10, "offset": 0}),
        };

        let resp = dispatch(&repo, &req, Path::new("."));
        assert_yaml_snapshot!("detect_undocumented_symbols_response", resp);
    }

    #[test]
    fn sample_data_uses_valid_types() {
        let _ = ContentHash::from_hex("aabbccdd".to_owned());
        let _ = Language::TypeScript;
    }
}
