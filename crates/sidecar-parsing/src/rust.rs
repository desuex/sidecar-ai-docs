use std::cell::RefCell;

use tree_sitter::{Node, Parser};

use crate::adapter::{LanguageAdapter, RawRef, RawSymbol};
use sidecar_types::{Language, Range, SymbolKind, Visibility};

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

    fn parse_refs(&self, _source: &[u8]) -> Vec<RawRef> {
        // TODO(M2): Rust reference extraction
        Vec::new()
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
