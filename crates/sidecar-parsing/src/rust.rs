use std::cell::RefCell;

use tree_sitter::{Node, Parser};

use crate::adapter::{LanguageAdapter, RawRef, RawSymbol};
use sidecar_types::{Language, Range, RefKind, SymbolKind, Visibility};

/// Rust language adapter using Tree-sitter.
pub struct RustAdapter {
    parser: RefCell<Parser>,
}

// Safety: same rationale as TypeScriptAdapter — single-threaded usage via RefCell.
unsafe impl Send for RustAdapter {}
unsafe impl Sync for RustAdapter {}

impl RustAdapter {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .expect("Error loading Rust parser");
        RustAdapter {
            parser: RefCell::new(parser),
        }
    }
}

impl Default for RustAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageAdapter for RustAdapter {
    fn language(&self) -> Language {
        Language::Rust
    }

    fn parse_symbols(&self, source: &[u8]) -> Vec<RawSymbol> {
        let tree = match self.parser.borrow_mut().parse(source, None) {
            Some(t) => t,
            None => return Vec::new(),
        };

        let mut symbols = Vec::new();
        let root = tree.root_node();
        collect_symbols(source, &root, None, &mut symbols);

        symbols.sort_by(|a, b| a.range.start.cmp(&b.range.start));
        symbols
    }

    fn parse_refs(&self, source: &[u8]) -> Vec<RawRef> {
        let tree = match self.parser.borrow_mut().parse(source, None) {
            Some(t) => t,
            None => return Vec::new(),
        };

        let mut refs = Vec::new();
        let root = tree.root_node();
        collect_refs(source, &root, None, &mut refs);

        refs.sort_by(|a, b| a.range.start.cmp(&b.range.start));
        refs
    }
}

fn collect_symbols(
    source: &[u8],
    node: &Node,
    parent_name: Option<&str>,
    out: &mut Vec<RawSymbol>,
) {
    let kind_str = node.kind();

    match kind_str {
        "function_item" => {
            if let Some(sym) = extract_function(source, node, parent_name) {
                out.push(sym);
                return;
            }
        }
        "struct_item" => {
            if let Some(sym) = extract_named_item(source, node, SymbolKind::Class) {
                out.push(sym);
                return;
            }
        }
        "enum_item" => {
            if let Some(sym) = extract_named_item(source, node, SymbolKind::Enum) {
                out.push(sym);
                return;
            }
        }
        "trait_item" => {
            if let Some(sym) = extract_named_item(source, node, SymbolKind::Interface) {
                out.push(sym);
                return;
            }
        }
        "type_item" => {
            if let Some(sym) = extract_named_item(source, node, SymbolKind::Type) {
                out.push(sym);
                return;
            }
        }
        "const_item" => {
            if let Some(sym) = extract_named_item(source, node, SymbolKind::Constant) {
                out.push(sym);
                return;
            }
        }
        "static_item" => {
            if let Some(sym) = extract_named_item(source, node, SymbolKind::Variable) {
                out.push(sym);
                return;
            }
        }
        "mod_item" => {
            if let Some(sym) = extract_named_item(source, node, SymbolKind::Module) {
                out.push(sym);
                return;
            }
        }
        "impl_item" => {
            // Extract methods from impl block
            let impl_type = extract_impl_type_name(source, node);
            if let Some(body) = node.child_by_field_name("body") {
                for i in 0..body.child_count() {
                    if let Some(child) = body.child(i) {
                        collect_symbols(source, &child, impl_type.as_deref(), out);
                    }
                }
            }
            return;
        }
        _ => {}
    }

    // Recurse into children for unmatched nodes
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            collect_symbols(source, &child, parent_name, out);
        }
    }
}

fn node_text<'a>(source: &'a [u8], node: &Node) -> &'a str {
    std::str::from_utf8(&source[node.start_byte()..node.end_byte()]).unwrap_or("")
}

/// Check if node has a `visibility_modifier` child (i.e., `pub`).
fn is_pub(node: &Node) -> bool {
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.kind() == "visibility_modifier" {
                return true;
            }
        }
    }
    false
}

fn extract_function(source: &[u8], node: &Node, parent_name: Option<&str>) -> Option<RawSymbol> {
    let name_node = node.child_by_field_name("name")?;
    let name = node_text(source, &name_node).to_owned();

    let (kind, qualified_name) = if let Some(parent) = parent_name {
        (SymbolKind::Method, format!("{parent}.{name}"))
    } else {
        (SymbolKind::Function, name.clone())
    };

    let params = extract_params_text(source, node);
    let parent_str = parent_name.unwrap_or("");
    let fingerprint_input =
        format!("function_item\nname: {name}\nparent: {parent_str}\nparameters: [{params}]");

    Some(RawSymbol {
        qualified_name,
        name,
        kind,
        visibility: if is_pub(node) {
            Visibility::Public
        } else {
            Visibility::Private
        },
        range: Range {
            start: node.start_byte() as u32,
            end: node.end_byte() as u32,
        },
        fingerprint_input,
    })
}

fn extract_named_item(source: &[u8], node: &Node, kind: SymbolKind) -> Option<RawSymbol> {
    let name_node = node.child_by_field_name("name")?;
    let name = node_text(source, &name_node).to_owned();
    let kind_str = node.kind();
    let fingerprint_input = format!("{kind_str}\nname: {name}");

    Some(RawSymbol {
        qualified_name: name.clone(),
        name,
        kind,
        visibility: if is_pub(node) {
            Visibility::Public
        } else {
            Visibility::Private
        },
        range: Range {
            start: node.start_byte() as u32,
            end: node.end_byte() as u32,
        },
        fingerprint_input,
    })
}

/// Extract the type name from an `impl_item` (e.g., `impl Foo` → "Foo").
fn extract_impl_type_name(source: &[u8], node: &Node) -> Option<String> {
    // The type being implemented is the `type` field
    if let Some(type_node) = node.child_by_field_name("type") {
        return Some(node_text(source, &type_node).to_owned());
    }
    // Fallback: look for type_identifier child
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.kind() == "type_identifier" {
                return Some(node_text(source, &child).to_owned());
            }
        }
    }
    None
}

/// Extract parameter text for fingerprint.
fn extract_params_text(source: &[u8], node: &Node) -> String {
    if let Some(params) = node.child_by_field_name("parameters") {
        let text = node_text(source, &params);
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    } else {
        String::new()
    }
}

/// Recursively walk the AST and collect references.
fn collect_refs(source: &[u8], node: &Node, enclosing: Option<&str>, out: &mut Vec<RawRef>) {
    let kind_str = node.kind();

    match kind_str {
        // use foo::bar::{Baz, Qux};
        "use_declaration" => {
            collect_use_names(source, node, out);
            return;
        }
        // function call: foo(), self.bar(), Foo::new()
        "call_expression" => {
            if let Some(func) = node.child_by_field_name("function") {
                let call_name = extract_rust_call_name(source, &func);
                if !call_name.is_empty() && !is_rust_builtin(&call_name) {
                    let from = enclosing.unwrap_or("<file>").to_owned();
                    out.push(RawRef {
                        from_qualified_name: from,
                        to_name: call_name,
                        range: Range {
                            start: node.start_byte() as u32,
                            end: node.end_byte() as u32,
                        },
                        ref_kind: RefKind::Call,
                    });
                }
            }
        }
        // Type references in type positions
        "type_identifier" => {
            let name = node_text(source, node).to_owned();
            if !is_rust_primitive_type(&name) {
                let from = enclosing.unwrap_or("<file>").to_owned();
                out.push(RawRef {
                    from_qualified_name: from,
                    to_name: name,
                    range: Range {
                        start: node.start_byte() as u32,
                        end: node.end_byte() as u32,
                    },
                    ref_kind: RefKind::TypeRef,
                });
            }
            return;
        }
        // impl Trait for Type — trait is an Inherit ref
        "impl_item" => {
            // Check for trait name
            if let Some(trait_node) = node.child_by_field_name("trait") {
                let trait_name = node_text(source, &trait_node).to_owned();
                let impl_type = extract_impl_type_name(source, node);
                let from = impl_type.as_deref().unwrap_or("<file>");
                out.push(RawRef {
                    from_qualified_name: from.to_owned(),
                    to_name: trait_name,
                    range: Range {
                        start: node.start_byte() as u32,
                        end: node.end_byte() as u32,
                    },
                    ref_kind: RefKind::Inherit,
                });
            }
            // Recurse into impl body with type name as enclosing
            let impl_type = extract_impl_type_name(source, node);
            if let Some(body) = node.child_by_field_name("body") {
                for i in 0..body.child_count() {
                    if let Some(child) = body.child(i) {
                        collect_refs(source, &child, impl_type.as_deref().or(enclosing), out);
                    }
                }
            }
            return;
        }
        // Track enclosing scope
        "function_item" => {
            let fn_name = node.child_by_field_name("name").map(|n| {
                let name = node_text(source, &n);
                match enclosing {
                    Some(parent) => format!("{parent}.{name}"),
                    None => name.to_owned(),
                }
            });
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    collect_refs(source, &child, fn_name.as_deref().or(enclosing), out);
                }
            }
            return;
        }
        _ => {}
    }

    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            collect_refs(source, &child, enclosing, out);
        }
    }
}

fn collect_use_names(source: &[u8], node: &Node, out: &mut Vec<RawRef>) {
    // Walk the use tree to find identifiers
    let cursor = node.walk();
    let mut found_names = Vec::new();
    walk_use_tree(source, node, &mut found_names);
    for name in found_names {
        out.push(RawRef {
            from_qualified_name: "<file>".to_owned(),
            to_name: name,
            range: Range {
                start: node.start_byte() as u32,
                end: node.end_byte() as u32,
            },
            ref_kind: RefKind::Import,
        });
    }
    let _ = cursor;
}

fn walk_use_tree(source: &[u8], node: &Node, names: &mut Vec<String>) {
    match node.kind() {
        "use_as_clause" | "use_wildcard" => {
            // use foo as bar — the identifier is the thing being imported
            if let Some(path) = node.child_by_field_name("path") {
                if let Some(last) = last_path_segment(source, &path) {
                    names.push(last);
                }
            }
        }
        "use_list" => {
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    walk_use_tree(source, &child, names);
                }
            }
        }
        "scoped_use_list" => {
            if let Some(list) = node.child_by_field_name("list") {
                walk_use_tree(source, &list, names);
            }
        }
        "scoped_identifier" => {
            if let Some(name_node) = node.child_by_field_name("name") {
                let name = node_text(source, &name_node);
                if name != "self" && name != "super" && name != "crate" {
                    names.push(name.to_owned());
                }
            }
        }
        "identifier" => {
            let name = node_text(source, node);
            if name != "self" && name != "super" && name != "crate" && name != "use" {
                names.push(name.to_owned());
            }
        }
        _ => {
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    walk_use_tree(source, &child, names);
                }
            }
        }
    }
}

fn last_path_segment(source: &[u8], node: &Node) -> Option<String> {
    if node.kind() == "scoped_identifier" {
        node.child_by_field_name("name")
            .map(|n| node_text(source, &n).to_owned())
    } else if node.kind() == "identifier" {
        Some(node_text(source, node).to_owned())
    } else {
        None
    }
}

fn extract_rust_call_name(source: &[u8], func_node: &Node) -> String {
    match func_node.kind() {
        "identifier" => node_text(source, func_node).to_owned(),
        "field_expression" => {
            // self.method() or obj.method()
            if let Some(field) = func_node.child_by_field_name("field") {
                node_text(source, &field).to_owned()
            } else {
                String::new()
            }
        }
        "scoped_identifier" => {
            // Foo::new() — extract "new" or the full path
            if let Some(name) = func_node.child_by_field_name("name") {
                node_text(source, &name).to_owned()
            } else {
                String::new()
            }
        }
        _ => String::new(),
    }
}

fn is_rust_builtin(name: &str) -> bool {
    matches!(
        name,
        "println"
            | "eprintln"
            | "print"
            | "eprint"
            | "format"
            | "write"
            | "writeln"
            | "panic"
            | "todo"
            | "unimplemented"
            | "unreachable"
            | "vec"
            | "assert"
            | "assert_eq"
            | "assert_ne"
            | "debug_assert"
    )
}

fn is_rust_primitive_type(name: &str) -> bool {
    matches!(
        name,
        "bool"
            | "char"
            | "str"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
            | "f32"
            | "f64"
            | "Self"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use sidecar_types::RefKind;

    const SAMPLE_RS: &[u8] = br#"
pub struct CartService {
    items: Vec<String>,
}

impl CartService {
    pub fn new() -> Self {
        CartService { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
    }

    fn internal_helper(&self) -> usize {
        self.items.len()
    }
}

pub fn create_cart() -> CartService {
    CartService::new()
}

pub trait Repository {
    fn get_symbol(&self, uid: &str) -> Option<String>;
}

pub enum Status {
    Active,
    Inactive,
}

pub type CartId = u64;

const MAX_ITEMS: usize = 100;

pub mod utils {
    pub fn helper() {}
}
"#;

    #[test]
    fn extracts_rust_symbols() {
        let adapter = RustAdapter::new();
        let symbols = adapter.parse_symbols(SAMPLE_RS);

        let names: Vec<&str> = symbols.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"CartService"), "missing struct: {names:?}");
        assert!(names.contains(&"new"), "missing method: {names:?}");
        assert!(names.contains(&"add_item"), "missing method: {names:?}");
        assert!(
            names.contains(&"internal_helper"),
            "missing private method: {names:?}"
        );
        assert!(
            names.contains(&"create_cart"),
            "missing function: {names:?}"
        );
        assert!(names.contains(&"Repository"), "missing trait: {names:?}");
        assert!(names.contains(&"Status"), "missing enum: {names:?}");
        assert!(names.contains(&"CartId"), "missing type alias: {names:?}");
        assert!(names.contains(&"MAX_ITEMS"), "missing const: {names:?}");
        assert!(names.contains(&"utils"), "missing module: {names:?}");
    }

    #[test]
    fn qualified_names_for_methods() {
        let adapter = RustAdapter::new();
        let symbols = adapter.parse_symbols(SAMPLE_RS);

        let add = symbols
            .iter()
            .find(|s| s.name == "add_item")
            .expect("add_item not found");
        assert_eq!(add.qualified_name, "CartService.add_item");
        assert_eq!(add.kind, SymbolKind::Method);
    }

    #[test]
    fn visibility_detection() {
        let adapter = RustAdapter::new();
        let symbols = adapter.parse_symbols(SAMPLE_RS);

        let cart = symbols.iter().find(|s| s.name == "CartService").unwrap();
        assert_eq!(cart.visibility, Visibility::Public);

        let max = symbols.iter().find(|s| s.name == "MAX_ITEMS").unwrap();
        assert_eq!(max.visibility, Visibility::Private);

        let helper = symbols
            .iter()
            .find(|s| s.name == "internal_helper")
            .unwrap();
        assert_eq!(helper.visibility, Visibility::Private);
    }

    #[test]
    fn deterministic_output() {
        let adapter = RustAdapter::new();
        let s1 = adapter.parse_symbols(SAMPLE_RS);
        let s2 = adapter.parse_symbols(SAMPLE_RS);

        let names1: Vec<&str> = s1.iter().map(|s| s.name.as_str()).collect();
        let names2: Vec<&str> = s2.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names1, names2);
    }

    #[test]
    fn sorted_by_range() {
        let adapter = RustAdapter::new();
        let symbols = adapter.parse_symbols(SAMPLE_RS);

        for window in symbols.windows(2) {
            assert!(
                window[0].range.start <= window[1].range.start,
                "symbols not sorted by range"
            );
        }
    }

    const REFS_RS: &[u8] = br#"
use std::collections::HashMap;
use crate::model::Symbol;

pub struct Indexer {
    symbols: Vec<Symbol>,
}

impl Indexer {
    pub fn new() -> Self {
        let map = HashMap::new();
        Indexer { symbols: Vec::new() }
    }

    pub fn index(&self) -> usize {
        self.symbols.len()
    }
}

impl Default for Indexer {
    fn default() -> Self {
        Self::new()
    }
}

pub fn run_indexer(indexer: &Indexer) -> usize {
    indexer.index()
}
"#;

    #[test]
    fn extracts_rust_use_refs() {
        let adapter = RustAdapter::new();
        let refs = adapter.parse_refs(REFS_RS);
        let imports: Vec<&str> = refs
            .iter()
            .filter(|r| r.ref_kind == RefKind::Import)
            .map(|r| r.to_name.as_str())
            .collect();
        assert!(
            imports.contains(&"HashMap"),
            "missing HashMap import: {imports:?}"
        );
        assert!(
            imports.contains(&"Symbol"),
            "missing Symbol import: {imports:?}"
        );
    }

    #[test]
    fn extracts_rust_call_refs() {
        let adapter = RustAdapter::new();
        let refs = adapter.parse_refs(REFS_RS);
        let calls: Vec<&str> = refs
            .iter()
            .filter(|r| r.ref_kind == RefKind::Call)
            .map(|r| r.to_name.as_str())
            .collect();
        assert!(calls.contains(&"new"), "missing new() call: {calls:?}");
        assert!(calls.contains(&"len"), "missing len() call: {calls:?}");
        assert!(calls.contains(&"index"), "missing index() call: {calls:?}");
    }

    #[test]
    fn extracts_rust_type_refs() {
        let adapter = RustAdapter::new();
        let refs = adapter.parse_refs(REFS_RS);
        let types: Vec<&str> = refs
            .iter()
            .filter(|r| r.ref_kind == RefKind::TypeRef)
            .map(|r| r.to_name.as_str())
            .collect();
        assert!(
            types.contains(&"Symbol"),
            "missing Symbol type ref: {types:?}"
        );
        assert!(
            types.contains(&"Indexer"),
            "missing Indexer type ref: {types:?}"
        );
    }

    #[test]
    fn extracts_rust_inherit_refs() {
        let adapter = RustAdapter::new();
        let refs = adapter.parse_refs(REFS_RS);
        let inherits: Vec<&str> = refs
            .iter()
            .filter(|r| r.ref_kind == RefKind::Inherit)
            .map(|r| r.to_name.as_str())
            .collect();
        assert!(
            inherits.contains(&"Default"),
            "missing Default impl: {inherits:?}"
        );
    }
}
