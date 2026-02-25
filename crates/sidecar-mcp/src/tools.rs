use std::path::Path;

use serde_json::Value;
use sidecar_core::query::{RefsQuery, SearchQuery};
use sidecar_core::{doc_parser, Repository};
use sidecar_types::{Limit, Offset, Uid};

use crate::protocol::{JsonRpcRequest, JsonRpcResponse};

/// Dispatch an MCP request to the appropriate tool handler.
pub fn dispatch<R: Repository>(repo: &R, req: &JsonRpcRequest, root: &Path) -> JsonRpcResponse {
    if req.jsonrpc != "2.0" {
        return JsonRpcResponse::error(req.id.clone(), -32600, "Invalid Request".to_owned());
    }

    match req.method.as_str() {
        "search_symbols" => handle_search_symbols(repo, req),
        "get_symbol" => handle_get_symbol(repo, req),
        "find_references" => handle_find_references(repo, req),
        "get_documentation" => handle_get_documentation(repo, req, root),
        _ => JsonRpcResponse::error(
            req.id.clone(),
            -32601,
            format!("Method not found: {}", req.method),
        ),
    }
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

#[cfg(test)]
mod tests {
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

        fn search_symbols(&self, _query: &SearchQuery) -> Result<SearchResult, SidecarError> {
            Ok(SearchResult {
                results: self.search_results.clone(),
                truncated: self.search_truncated,
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

        fn get_doc(&self, _uid: &Uid) -> Result<Option<DocRecord>, SidecarError> {
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
    fn sample_data_uses_valid_types() {
        let _ = ContentHash::from_hex("aabbccdd".to_owned());
        let _ = Language::TypeScript;
    }
}
